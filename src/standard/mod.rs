
pub mod special_funcs;
pub mod sh;

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
        res
    };

    pub static ref Default_Namespace: HashMap<GlobalConstantDescriptor, ObjectRef> = {
        let mut res: HashMap<GlobalConstantDescriptor, ObjectRef> = HashMap::new();
        res.insert((STANDARD_CONTEXT_ID, 0), Arc::new(special_funcs::Print));
        res.insert((STANDARD_CONTEXT_ID, 1), Arc::new(special_funcs::Exit));
        res.insert((STANDARD_CONTEXT_ID, 2), Arc::new(special_funcs::RangeFunc));
        res.insert((STANDARD_CONTEXT_ID, 3), Arc::new(sh::Sh));
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
