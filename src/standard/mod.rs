
pub mod special_funcs;
pub mod sh;
pub mod math;

use crate::core::*;
use std::sync::Arc;
use crate::bytecode::{GlobalContext, GlobalConstantDescriptor, ContextId, FrameId};
use std::collections::HashMap;
use crate::error::*;

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

            fn call(&self, $args: &[ObjectRef], _locals: &mut HashMap<NonLocalName, ObjectRef>) -> RuntimeResult<ObjectRef> {
                if !$args_range.contains(&$args.len()) {
                    return Err(RuntimeError::type_error(format!("Incorrect number of arguments: expected {:?}, got {}", $args_range, $args.len())));
                }
                $call
            }
        }
    };
}
