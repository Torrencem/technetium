
use crate::core::*;
use crate::builtins::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bytecode::NonLocalName;
use crate::bytecode::{ContextId, FrameId};
use crate::error::*;

use std::io::{self, Write};
use std::process::{Command, Child, Stdio, Output};

use crate::func_object;

#[derive(Debug)]
pub struct ShObject {
    pub argument: String,
    pub state: Mutex<ShObjectState>,
    pub child: Mutex<Option<Child>>,
    pub output: Mutex<Option<Output>>,
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
            state: Mutex::new(ShObjectState::Prepared),
            child: Mutex::new(None),
            output: Mutex::new(None),
        })
    }

    pub fn spawn(&self) -> io::Result<()> {
        trace!("Spawning subprocess from sh()");
        let mut state_ = self.state.lock().unwrap();
        let mut child_ = self.child.lock().unwrap();
        let mut cmd = Command::new("sh")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()?;
        cmd.stdin.as_mut().unwrap().write_all(self.argument.clone().as_bytes());
        if let ShObjectState::Prepared = *state_ {
            *state_ = ShObjectState::Running;
            *child_ = Some(
                cmd
            );
        }
        Ok(())
    }

    pub fn join(&self) -> io::Result<()> {
        trace!("Joining subprocess from sh(..).join()");
        let state_ = self.state.lock().unwrap();
        if let ShObjectState::Prepared = *state_ {
            drop(state_);
            self.spawn()?;
        }
        let mut state_ = self.state.lock().unwrap();
        let mut child_ = self.child.lock().unwrap();

        if let ShObjectState::Running = *state_ {
            *state_ = ShObjectState::Finished;
            let child_ = child_.take().unwrap();
            let mut output_ = self.output.lock().unwrap();
            *output_ = Some(child_.wait_with_output()?);
        }

        Ok(())
    }

    pub fn stdout(&self) -> io::Result<ObjectRef> {
        let output = self.output.lock().unwrap();
        if let Some(ref output) = *output {
            let bytes = &output.stdout;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned()
            ))
        } else {
            Ok(StringObject::new("".to_string()))
        }
    }
    
    pub fn stderr(&self) -> io::Result<ObjectRef> {
        let output = self.output.lock().unwrap();
        if let Some(ref output) = *output {
            let bytes = &output.stderr;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned()
            ))
        } else {
            Ok(StringObject::new("".to_string()))
        }
    }
    
    pub fn exit_code(&self) -> io::Result<ObjectRef> {
        let output = self.output.lock().unwrap();
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
            return Err(RuntimeError::type_error("Unexpected arguments to method call"));
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
        let val = str_obj.val.lock().unwrap();
        Ok(ShObject::new(val.clone()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sh; expected string"))
    }
});
