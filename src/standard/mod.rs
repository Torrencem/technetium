pub mod math;
pub mod sh;
pub mod special_funcs;
pub mod conversion;

use crate::bytecode::{ContextId, FrameId, GlobalConstantDescriptor, GlobalContext};
use crate::core::*;
use crate::error::*;
use std::collections::HashMap;
use std::sync::Arc;

pub static STANDARD_CONTEXT_ID: ContextId = 0;

lazy_static! {
    pub static ref Default_Namespace_Descriptors: HashMap<String, GlobalConstantDescriptor> = {
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
        res
    };
    pub static ref Default_Namespace: HashMap<GlobalConstantDescriptor, ObjectRef> = {
        let mut res: HashMap<GlobalConstantDescriptor, ObjectRef> = HashMap::new();
        res.insert((STANDARD_CONTEXT_ID, 0), Arc::new(special_funcs::Print));
        res.insert((STANDARD_CONTEXT_ID, 1), Arc::new(special_funcs::Exit));
        res.insert((STANDARD_CONTEXT_ID, 2), Arc::new(special_funcs::RangeFunc));
        res.insert((STANDARD_CONTEXT_ID, 3), Arc::new(sh::Sh));
        res.insert((STANDARD_CONTEXT_ID, 4), Arc::new(special_funcs::Cd));
        res.insert((STANDARD_CONTEXT_ID, 5), Arc::new(special_funcs::Os));
        res.insert((STANDARD_CONTEXT_ID, 6), Arc::new(special_funcs::LinuxDistro));
        res.insert((STANDARD_CONTEXT_ID, 7), Arc::new(math::Sin));
        res.insert((STANDARD_CONTEXT_ID, 8), Arc::new(math::Cos));
        res.insert((STANDARD_CONTEXT_ID, 9), Arc::new(math::Tan));
        res.insert((STANDARD_CONTEXT_ID, 10), Arc::new(math::Abs));
        res.insert((STANDARD_CONTEXT_ID, 11), Arc::new(math::Sqrt));
        res.insert((STANDARD_CONTEXT_ID, 12), Arc::new(math::Exp));
        res.insert((STANDARD_CONTEXT_ID, 13), Arc::new(math::Ln));
        res.insert((STANDARD_CONTEXT_ID, 14), Arc::new(math::Arcsin));
        res.insert((STANDARD_CONTEXT_ID, 15), Arc::new(math::Arccos));
        res.insert((STANDARD_CONTEXT_ID, 16), Arc::new(math::Arctan));
        res.insert((STANDARD_CONTEXT_ID, 17), Arc::new(special_funcs::Type));
        res.insert((STANDARD_CONTEXT_ID, 18), Arc::new(conversion::String_));
        res.insert((STANDARD_CONTEXT_ID, 19), Arc::new(special_funcs::Clone_));
        res.insert((STANDARD_CONTEXT_ID, 20), Arc::new(conversion::Bool));
        res.insert((STANDARD_CONTEXT_ID, 21), Arc::new(conversion::Int));
        res.insert((STANDARD_CONTEXT_ID, 22), Arc::new(conversion::Float));
        res
    };
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
