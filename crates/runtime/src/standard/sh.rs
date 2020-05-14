use crate::*;
use crate::error::*;

use std::io::Write;
use std::process::{Child, Command, Output, Stdio};
use std::path::Path;
use std::env;

use crate::func_object;

use sys_info::linux_os_release;
use sys_info::os_type;

#[derive(Debug)]
pub struct ShObject {
    pub argument: String,
    pub state: ShObjectState,
    pub child: Option<Child>,
    pub output: Option<Output>,
}

#[derive(Debug)]
pub enum ShObjectState {
    Prepared,
    Running,
    Finished,
}

impl ShObject {
    pub fn new(command: String) -> ObjectRef {
        ObjectRef::new(ShObject {
            argument: command,
            state: ShObjectState::Prepared,
            child: None,
            output: None,
        })
    }

    pub fn spawn(&mut self) -> RuntimeResult<()> {
        let mut cmd = Command::new("sh")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        cmd.stdin
            .as_mut()
            .unwrap()
            .write_all(self.argument.clone().as_bytes())?;
        if let ShObjectState::Prepared = self.state {
            self.state = ShObjectState::Running;
            self.child = Some(cmd);
        }
        Ok(())
    }

    pub fn join(&mut self) -> RuntimeResult<()> {
        if let ShObjectState::Prepared = self.state {
            self.spawn()?;
        }

        if let ShObjectState::Running = self.state {
            self.state = ShObjectState::Finished;
            let child = self.child.take().unwrap();
            self.output = Some(child.wait_with_output()?);
        }

        Ok(())
    }

    pub fn stdout(&self) -> RuntimeResult<ObjectRef> {
        if let Some(ref output) = self.output {
            let bytes = &output.stdout;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned(),
            ))
        } else {
            // TODO: Should this be like this?
            Ok(StringObject::new("".to_string()))
        }
    }

    pub fn stderr(&self) -> RuntimeResult<ObjectRef> {
        if let Some(ref output) = self.output {
            let bytes = &output.stderr;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned(),
            ))
        } else {
            Ok(StringObject::new("".to_string()))
        }
    }

    pub fn exit_code(&self) -> RuntimeResult<ObjectRef> {
        if let Some(ref output) = self.output {
            let status = &output.status;
            if let Some(val) = status.code() {
                Ok(IntObject::new(val as i64))
            } else {
                Ok(BoolObject::new(status.success()))
            }
        } else {
            Err(RuntimeError::type_error("Called exit_code() before process exited!"))
        }
    }

    pub fn kill(&mut self) -> RuntimeResult<()> {
        if let Some(ref mut child) = self.child {
            child.kill()?;
            Ok(())
        } else {
            Err(RuntimeError::type_error("Called kill() on process that wasn't running!"))
        }
    }
}

impl Object for ObjectCell<ShObject> {
    fn technetium_type_name(&self) -> String {
        "sh".to_string()
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        let mut this = self.try_borrow_mut()?;
        if args.len() != 0 {
            return Err(RuntimeError::type_error(
                "Unexpected arguments to method call",
            ));
        }

        match method {
            "spawn" => this.spawn()?,
            "join" => this.join()?,
            "stdout" => return Ok(this.stdout()?),
            "stderr" => return Ok(this.stderr()?),
            "exit_code" => return Ok(this.exit_code()?),
            "kill" => this.kill()?,
            _ => return Err(RuntimeError::type_error("Unknown method")),
        }

        Ok(VoidObject::new())
    }
}

func_object!(Sh, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;
        Ok(ShObject::new(val.clone()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sh; expected string"))
    }
});

func_object!(Cd, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;
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
