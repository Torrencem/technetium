use crate::builtins::*;
use crate::bytecode::NonLocalName;
use crate::bytecode::{ContextId, FrameId};
use crate::core::*;
use crate::error::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use std::io::{self, Write};
use std::process::{Child, Command, Output, Stdio};
use std::path::Path;
use std::env;

use crate::func_object;

use sys_info::linux_os_release;
use sys_info::os_type;

#[derive(Debug)]
pub struct ShObject {
    pub argument: String,
    pub state: RwLock<ShObjectState>,
    pub child: RwLock<Option<Child>>,
    pub output: RwLock<Option<Output>>,
}

#[derive(Debug)]
pub enum ShObjectState {
    Prepared,
    Running,
    Finished,
}

impl ShObject {
    pub fn new(command: String) -> ObjectRef {
        Arc::new(ShObject {
            argument: command,
            state: RwLock::new(ShObjectState::Prepared),
            child: RwLock::new(None),
            output: RwLock::new(None),
        })
    }

    pub fn spawn(&self) -> RuntimeResult<()> {
        trace!("Spawning subprocess from sh()");
        let mut state_ = self.state.write()?;
        let mut child_ = self.child.write()?;
        let mut cmd = Command::new("sh")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        cmd.stdin
            .as_mut()
            .unwrap()
            .write_all(self.argument.clone().as_bytes());
        if let ShObjectState::Prepared = *state_ {
            *state_ = ShObjectState::Running;
            *child_ = Some(cmd);
        }
        Ok(())
    }

    pub fn join(&self) -> RuntimeResult<()> {
        trace!("Joining subprocess from sh(..).join()");
        let state_ = self.state.read()?;
        if let ShObjectState::Prepared = *state_ {
            drop(state_);
            self.spawn()?;
        }
        let mut state_ = self.state.write()?;
        let mut child_ = self.child.write()?;

        if let ShObjectState::Running = *state_ {
            *state_ = ShObjectState::Finished;
            let child_ = child_.take().unwrap();
            let mut output_ = self.output.write()?;
            *output_ = Some(child_.wait_with_output()?);
        }

        Ok(())
    }

    pub fn stdout(&self) -> RuntimeResult<ObjectRef> {
        let output = self.output.read()?;
        if let Some(ref output) = *output {
            let bytes = &output.stdout;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned(),
            ))
        } else {
            Ok(StringObject::new("".to_string()))
        }
    }

    pub fn stderr(&self) -> RuntimeResult<ObjectRef> {
        let output = self.output.read()?;
        if let Some(ref output) = *output {
            let bytes = &output.stderr;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned(),
            ))
        } else {
            Ok(StringObject::new("".to_string()))
        }
    }

    pub fn exit_code(&self) -> RuntimeResult<ObjectRef> {
        let output = self.output.read()?;
        if let Some(ref output) = *output {
            let status = &output.status;
            if let Some(val) = status.code() {
                Ok(IntObject::new(val as i64))
            } else {
                Ok(BoolObject::new(status.success()))
            }
        } else {
            // TODO: Should this be like this?
            Ok(StringObject::new("".to_string()))
        }
    }
}

impl Object for ShObject {
    fn technetium_type_name(&self) -> String {
        "sh".to_string()
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        if args.len() != 0 {
            return Err(RuntimeError::type_error(
                "Unexpected arguments to method call",
            ));
        }

        match method {
            "spawn" => self.spawn()?,
            "join" => self.join()?,
            "stdout" => return Ok(self.stdout()?),
            "stderr" => return Ok(self.stderr()?),
            "exit_code" => return Ok(self.exit_code()?),
            _ => return Err(RuntimeError::type_error("Unknown method")),
        }

        Ok(VoidObject::new())
    }
}

func_object!(Sh, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<StringObject>() {
        let val = str_obj.val.read()?;
        Ok(ShObject::new(val.clone()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sh; expected string"))
    }
});

func_object!(Cd, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<StringObject>() {
        let val = str_obj.val.read()?;
        let path = Path::new(&*val);
        env::set_current_dir(path)?;
        Ok(VoidObject::new())
    } else {
        Err(RuntimeError::type_error("Expected string as argument to cd"))
    }
});

func_object!(Os, (0..=0), args -> {
    Ok(StringObject::new(os_type()?))
});

func_object!(LinuxDistro, (0..=0), args -> {
    Ok(StringObject::new(linux_os_release()?.name.unwrap_or("Unknown".to_string())))
});

