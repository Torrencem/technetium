use crate::error::*;
use crate::PARSED_CLARGS;
use crate::prelude::*;

use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};

use crate::func_object;

use sys_info::linux_os_release;
use sys_info::os_type;

use whoami::{hostname, devicename, realname, username, lang, desktop_env};

use opener::open;

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

    pub fn stdout(&mut self) -> RuntimeResult<ObjectRef> {
        if let Some(ref output) = self.output {
            let bytes = &output.stdout;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned(),
            ))
        } else {
            self.join()?;
            self.stdout()
        }
    }

    pub fn stderr(&mut self) -> RuntimeResult<ObjectRef> {
        if let Some(ref output) = self.output {
            let bytes = &output.stderr;
            Ok(StringObject::new(
                String::from_utf8_lossy(bytes).into_owned(),
            ))
        } else {
            self.join()?;
            self.stderr()
        }
    }

    pub fn exit_code(&mut self) -> RuntimeResult<ObjectRef> {
        if let Some(ref output) = self.output {
            let status = &output.status;
            if let Some(val) = status.code() {
                Ok(IntObject::new(val as i64))
            } else {
                Ok(BoolObject::new(status.success()))
            }
        } else {
            self.join()?;
            self.exit_code()
        }
    }

    pub fn kill(&mut self) -> RuntimeResult<()> {
        if let Some(ref mut child) = self.child {
            child.kill()?;
            Ok(())
        } else {
            Err(RuntimeError::type_error(
                "Called kill() on process that wasn't running!",
            ))
        }
    }
}

impl Object for ObjectCell<ShObject> {
    fn technetium_type_name(&self) -> String {
        "sh".to_string()
    }

    fn call_method(&self, method: &str, args: &[ObjectRef], _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let mut this = self.try_borrow_mut()?;
        if !args.is_empty() {
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

        Ok(UnitObject::new())
    }
}

func_object!(Sh, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;
        Ok(ShObject::new(val.clone()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sh; expected string"))
    }
});

func_object!(Cd, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;
        let path = Path::new(&*val);
        env::set_current_dir(path)?;
        Ok(UnitObject::new())
    } else {
        Err(RuntimeError::type_error("Expected string as argument to cd"))
    }
});

func_object_void!(Open, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;
        open(val)?;
    } else {
        return Err(RuntimeError::type_error("Expected string as argument to open"));
    }
});

func_object!(Os, (0..=0), _c, args -> {
    Ok(StringObject::new(os_type()?))
});

func_object!(LinuxDistro, (0..=0), _c, args -> {
    Ok(StringObject::new(linux_os_release()?.name.unwrap_or_else(|| "Unknown".to_string())))
});

func_object!(Args, (0..=0), _c, args -> {
    let mut res = vec![];
    for val in PARSED_CLARGS.get().unwrap().iter() {
        res.push(StringObject::new(val.clone()));
    }
    Ok(ObjectRef::new(List { contents: res }))
});

func_object!(ScriptPath, (0..=0), _c, args -> {
    let path = match crate::CURR_SCRIPT_PATH.get() {
        Some(path) => {
            path.to_str().unwrap().to_string()
        },
        None => {
            return Err(RuntimeError::type_error("Cannot call script_path when source of code is not a file"));
        }
    };
    Ok(StringObject::new(path))
});

func_object!(Which, (1..=1), _c, args -> {
    let result = which::which(args[0].to_string()?)
        .map_err(|e| RuntimeError::child_process_error(e.to_string()))?;

    Ok(StringObject::new(result.to_string_lossy().into_owned()))
});

func_object!(Exists, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;

        let p = PathBuf::from(val);

        Ok(BoolObject::new(p.exists()))
    } else {
        return Err(RuntimeError::type_error("Expected string as argument to exists"));
    }
});

func_object!(IsDirectory, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;

        let p = PathBuf::from(val);

        Ok(BoolObject::new(p.is_dir()))
    } else {
        return Err(RuntimeError::type_error("Expected string as argument to is_directory"));
    }
});

func_object!(Canonicalize, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<ObjectCell<StringObject>>() {
        let str_obj = str_obj.try_borrow()?;
        let val = &str_obj.val;

        let p = PathBuf::from(val);

        let canonicalized = p.canonicalize()?
            .to_string_lossy()
            .into_owned();

        Ok(StringObject::new(canonicalized))
    } else {
        return Err(RuntimeError::type_error("Expected string as argument to canonicalize"));
    }
});

func_object!(Hostname, (0..=0), _c, _args -> {
    let hostname = hostname();
    Ok(StringObject::new(hostname))
});

func_object!(Devicename, (0..=0), _c, _args -> {
    let devicename = devicename();
    Ok(StringObject::new(devicename))
});

func_object!(Realname, (0..=0), _c, _args -> {
    let realname = realname();
    Ok(StringObject::new(realname))
});

func_object!(Username, (0..=0), _c, _args -> {
    let username = username();
    Ok(StringObject::new(username))
});

func_object!(Langs, (0..=0), _c, _args -> {
    let langs = lang();
    let mut res = vec![];
    for val in langs.into_iter() {
        res.push(StringObject::new(val));
    }
    Ok(ObjectRef::new(List { contents: res }))
});

func_object!(DesktopEnv, (0..=0), _c, _args -> {
    let desktop_env = desktop_env();
    Ok(StringObject::new(desktop_env.to_string()))
});
