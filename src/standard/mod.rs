pub mod math;
pub mod sh;
pub mod special_funcs;
pub mod conversion;
pub mod string;

use crate::bytecode::{ContextId, FrameId, GlobalConstantDescriptor, GlobalContext};
use crate::core::*;
use crate::error::*;
use std::collections::HashMap;
use std::rc::Rc;

pub static STANDARD_CONTEXT_ID: ContextId = 0;

pub fn get_default_namespace_descriptors() -> HashMap<String, GlobalConstantDescriptor> {
        let mut res: HashMap<String, GlobalConstantDescriptor> = HashMap::new();
        res.insert("print".to_string(), (STANDARD_CONTEXT_ID, 0));
        res.insert("exit".to_string(), (STANDARD_CONTEXT_ID, 1));
        res.insert("range".to_string(), (STANDARD_CONTEXT_ID, 2));
        res.insert("sh".to_string(), (STANDARD_CONTEXT_ID, 3));
        res.insert("cd".to_string(), (STANDARD_CONTEXT_ID, 4));
        res.insert("os".to_string(), (STANDARD_CONTEXT_ID, 5));
        res.insert("linux_distro".to_string(), (STANDARD_CONTEXT_ID, 6));
        res.insert("sin".to_string(), (STANDARD_CONTEXT_ID, 7));
        res.insert("cos".to_string(), (STANDARD_CONTEXT_ID, 8));
        res.insert("tan".to_string(), (STANDARD_CONTEXT_ID, 9));
        res.insert("abs".to_string(), (STANDARD_CONTEXT_ID, 10));
        res.insert("sqrt".to_string(), (STANDARD_CONTEXT_ID, 11));
        res.insert("exp".to_string(), (STANDARD_CONTEXT_ID, 12));
        res.insert("ln".to_string(), (STANDARD_CONTEXT_ID, 13));
        res.insert("arcsin".to_string(), (STANDARD_CONTEXT_ID, 14));
        res.insert("arccos".to_string(), (STANDARD_CONTEXT_ID, 15));
        res.insert("arctan".to_string(), (STANDARD_CONTEXT_ID, 16));
        res.insert("type".to_string(), (STANDARD_CONTEXT_ID, 17));
        res.insert("string".to_string(), (STANDARD_CONTEXT_ID, 18));
        res.insert("clone".to_string(), (STANDARD_CONTEXT_ID, 19));
        res.insert("bool".to_string(), (STANDARD_CONTEXT_ID, 20));
        res.insert("int".to_string(), (STANDARD_CONTEXT_ID, 21));
        res.insert("float".to_string(), (STANDARD_CONTEXT_ID, 22));
        res.insert("char".to_string(), (STANDARD_CONTEXT_ID, 23));
        res
}

pub fn get_default_namespace() -> HashMap<GlobalConstantDescriptor, ObjectRef> {
        let mut res: HashMap<GlobalConstantDescriptor, ObjectRef> = HashMap::new();
        res.insert((STANDARD_CONTEXT_ID, 0), Rc::new(special_funcs::Print));
        res.insert((STANDARD_CONTEXT_ID, 1), Rc::new(special_funcs::Exit));
        res.insert((STANDARD_CONTEXT_ID, 2), Rc::new(special_funcs::RangeFunc));
        res.insert((STANDARD_CONTEXT_ID, 3), Rc::new(sh::Sh));
        res.insert((STANDARD_CONTEXT_ID, 4), Rc::new(sh::Cd));
        res.insert((STANDARD_CONTEXT_ID, 5), Rc::new(sh::Os));
        res.insert((STANDARD_CONTEXT_ID, 6), Rc::new(sh::LinuxDistro));
        res.insert((STANDARD_CONTEXT_ID, 7), Rc::new(math::Sin));
        res.insert((STANDARD_CONTEXT_ID, 8), Rc::new(math::Cos));
        res.insert((STANDARD_CONTEXT_ID, 9), Rc::new(math::Tan));
        res.insert((STANDARD_CONTEXT_ID, 10), Rc::new(math::Abs));
        res.insert((STANDARD_CONTEXT_ID, 11), Rc::new(math::Sqrt));
        res.insert((STANDARD_CONTEXT_ID, 12), Rc::new(math::Exp));
        res.insert((STANDARD_CONTEXT_ID, 13), Rc::new(math::Ln));
        res.insert((STANDARD_CONTEXT_ID, 14), Rc::new(math::Rcsin));
        res.insert((STANDARD_CONTEXT_ID, 15), Rc::new(math::Rccos));
        res.insert((STANDARD_CONTEXT_ID, 16), Rc::new(math::Rctan));
        res.insert((STANDARD_CONTEXT_ID, 17), Rc::new(special_funcs::Type));
        res.insert((STANDARD_CONTEXT_ID, 18), Rc::new(conversion::String_));
        res.insert((STANDARD_CONTEXT_ID, 19), Rc::new(special_funcs::Clone_));
        res.insert((STANDARD_CONTEXT_ID, 20), Rc::new(conversion::Bool));
        res.insert((STANDARD_CONTEXT_ID, 21), Rc::new(conversion::Int));
        res.insert((STANDARD_CONTEXT_ID, 22), Rc::new(conversion::Float));
        res.insert((STANDARD_CONTEXT_ID, 23), Rc::new(conversion::Char));
        res
}

#[macro_export]
macro_rules! func_object_void {
    ($id:ident, $args_range:tt, $args:ident -> $call:block) => {
        pub struct $id;

        impl Object for $id {
            fn technetium_type_name(&self) -> String {
                "builtin func".to_string()
            }

            fn call(&self, $args: &[ObjectRef], _locals: &mut HashMap<NonLocalName, ObjectRef>) -> RuntimeResult<ObjectRef> {
                if !$args_range.contains(&$args.len()) {
                    return Err(RuntimeError::type_error(format!("Incorrect number of arguments: expected {:?}, got {}", $args_range, $args.len())));
                }
                $call
                Ok(VoidObject::new())
            }
        }
    };
}

#[macro_export]
macro_rules! func_object {
    ($id:ident, $args_range:tt, $args:ident -> $call:block) => {
        pub struct $id;

        impl Object for $id {
            fn technetium_type_name(&self) -> String {
                "builtin func".to_string()
            }

            fn call(
                &self,
                $args: &[ObjectRef],
                _locals: &mut HashMap<NonLocalName, ObjectRef>,
            ) -> RuntimeResult<ObjectRef> {
                if !$args_range.contains(&$args.len()) {
                    return Err(RuntimeError::type_error(format!(
                        "Incorrect number of arguments: expected {:?}, got {}",
                        $args_range,
                        $args.len()
                    )));
                }
                $call
            }
        }
    };
}
