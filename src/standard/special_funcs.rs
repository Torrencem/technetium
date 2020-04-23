
use crate::core::*;
use crate::builtins::*;

use std::process::exit;

pub struct Print;

impl Object for Print {
    fn marsh_type_name(&self) -> String {
        "builtin func".to_string()
    }

    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef> {
        let mut first = true;
        for arg in args.iter() {
            if !first {
                print!("\t");
            } else {
                first = false;
            }
            print!("{}", arg.to_string()?);
        }
        println!();
        Ok(VoidObject::new())
    }
}

pub struct Exit;

impl Object for Exit {
    fn marsh_type_name(&self) -> String {
        "builtin func".to_string()
    }

    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef> {
        if args.len() != 1 {
            return Err(RuntimeError::type_error("exit expects 1 arg".to_string()));
        }
        let arg_any = args[0].as_any();
        if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
            exit(int_obj.val as i32)
        } else {
            exit(if args[0].truthy() { 1 } else { 0 })
        }
    }
}
