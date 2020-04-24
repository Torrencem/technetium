
pub mod special_funcs;

use crate::core::*;
use std::sync::Arc;
use crate::bytecode::{GlobalContext, GlobalConstantDescriptor};
use std::collections::HashMap;

lazy_static! {
    pub static ref Default_Namespace_Descriptors: HashMap<String, GlobalConstantDescriptor> = {
        let mut res: HashMap<String, GlobalConstantDescriptor> = HashMap::new();
        res.insert("print".to_string(), 0);
        res.insert("exit".to_string(), 1);
        res
    };

    pub static ref Default_Namespace: HashMap<GlobalConstantDescriptor, ObjectRef> = {
        let mut res: HashMap<GlobalConstantDescriptor, ObjectRef> = HashMap::new();
        res.insert(0, Arc::new(special_funcs::Print));
        res.insert(1, Arc::new(special_funcs::Exit));
        res
    };
}

#[macro_export]
macro_rules! func_object_void {
    ($id:ident, $args_range:tt, $args:ident -> $call:block) => {
        pub struct $id;

        impl Object for $id {
            fn marsh_type_name(&self) -> String {
                "builtin func".to_string()
            }

            fn call(&self, $args: &[ObjectRef], _locals: &mut HashMap<LocalName, ObjectRef>) -> Result<ObjectRef> {
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
            fn marsh_type_name(&self) -> String {
                "builtin func".to_string()
            }

            fn call(&self, $args: &[ObjectRef], _locals: &mut HashMap<LocalName, ObjectRef>) -> Result<ObjectRef> {
                if !$args_range.contains(&$args.len()) {
                    return Err(RuntimeError::type_error(format!("Incorrect number of arguments: expected {:?}, got {}", $args_range, $args.len())));
                }
                $call
            }
        }
    };
}
