//! The standard library. This module contains definitions for function objects
//! which are in scope throughout all technetium programs

pub mod conversion;
pub mod math;
pub mod sh;
pub mod special_funcs;
pub mod string;
pub mod functional;

use crate::bytecode::{ContextId, GlobalConstantDescriptor};
use crate::prelude::*;
use std::collections::HashMap;

pub static STANDARD_CONTEXT_ID: ContextId = 0;

pub fn get_default_namespace_descriptors() -> HashMap<String, GlobalConstantDescriptor> {
    let mut res: HashMap<String, GlobalConstantDescriptor> = HashMap::new();
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
    res.insert("hash".to_string(), (STANDARD_CONTEXT_ID, 24));
    res.insert("lock".to_string(), (STANDARD_CONTEXT_ID, 25));
    res.insert("list".to_string(), (STANDARD_CONTEXT_ID, 26));
    res.insert("set".to_string(), (STANDARD_CONTEXT_ID, 27));
    res.insert("args".to_string(), (STANDARD_CONTEXT_ID, 28));
    res.insert("which".to_string(), (STANDARD_CONTEXT_ID, 29));
    res.insert("map".to_string(), (STANDARD_CONTEXT_ID, 30));
    res.insert("filter".to_string(), (STANDARD_CONTEXT_ID, 31));
    res.insert("print".to_string(), (STANDARD_CONTEXT_ID, 32));
    res.insert("printr".to_string(), (STANDARD_CONTEXT_ID, 33));
    res.insert("println".to_string(), (STANDARD_CONTEXT_ID, 34));
    res.insert("eprint".to_string(), (STANDARD_CONTEXT_ID, 35));
    res.insert("eprintr".to_string(), (STANDARD_CONTEXT_ID, 36));
    res.insert("eprintln".to_string(), (STANDARD_CONTEXT_ID, 37));
    res.insert("open".to_string(), (STANDARD_CONTEXT_ID, 38));
    res.insert("exists".to_string(), (STANDARD_CONTEXT_ID, 39));
    res.insert("is_directory".to_string(), (STANDARD_CONTEXT_ID, 40));
    res.insert("canonicalize".to_string(), (STANDARD_CONTEXT_ID, 41));
    res.insert("dict".to_string(), (STANDARD_CONTEXT_ID, 42));
    res.insert("hostname".to_string(), (STANDARD_CONTEXT_ID, 43));
    res.insert("device_name".to_string(), (STANDARD_CONTEXT_ID, 44));
    res.insert("real_name".to_string(), (STANDARD_CONTEXT_ID, 45));
    res.insert("username".to_string(), (STANDARD_CONTEXT_ID, 46));
    res.insert("languages".to_string(), (STANDARD_CONTEXT_ID, 47));
    res.insert("desktop_env".to_string(), (STANDARD_CONTEXT_ID, 48));
    res.insert("assert".to_string(), (STANDARD_CONTEXT_ID, 49));
    res.insert("tech_version".to_string(), (STANDARD_CONTEXT_ID, 50));
    res.insert("stale".to_string(), (STANDARD_CONTEXT_ID, 51));
    res.insert("script_path".to_string(), (STANDARD_CONTEXT_ID, 52));
    res.insert("rand".to_string(), (STANDARD_CONTEXT_ID, 53));
    res.insert("rand_range".to_string(), (STANDARD_CONTEXT_ID, 54));
    res.insert("rand_normal".to_string(), (STANDARD_CONTEXT_ID, 55));
    res.insert("sleep".to_string(), (STANDARD_CONTEXT_ID, 56));
    res.insert("strip_path_prefix".to_string(), (STANDARD_CONTEXT_ID, 57));
    res.insert("strip_prefix".to_string(), (STANDARD_CONTEXT_ID, 58));
    res.insert("strip_suffix".to_string(), (STANDARD_CONTEXT_ID, 59));
    res
}

pub fn get_default_namespace() -> HashMap<GlobalConstantDescriptor, ObjectRef> {
    let mut res: HashMap<GlobalConstantDescriptor, ObjectRef> = HashMap::new();
    res.insert(
        (STANDARD_CONTEXT_ID, 1),
        ObjectRef::new(special_funcs::Exit),
    );
    res.insert(
        (STANDARD_CONTEXT_ID, 2),
        ObjectRef::new(special_funcs::RangeFunc),
    );
    res.insert((STANDARD_CONTEXT_ID, 3), ObjectRef::new(sh::Sh));
    res.insert((STANDARD_CONTEXT_ID, 4), ObjectRef::new(sh::Cd));
    res.insert((STANDARD_CONTEXT_ID, 5), ObjectRef::new(sh::Os));
    res.insert((STANDARD_CONTEXT_ID, 6), ObjectRef::new(sh::LinuxDistro));
    res.insert((STANDARD_CONTEXT_ID, 7), ObjectRef::new(math::Sin));
    res.insert((STANDARD_CONTEXT_ID, 8), ObjectRef::new(math::Cos));
    res.insert((STANDARD_CONTEXT_ID, 9), ObjectRef::new(math::Tan));
    res.insert((STANDARD_CONTEXT_ID, 10), ObjectRef::new(math::Abs));
    res.insert((STANDARD_CONTEXT_ID, 11), ObjectRef::new(math::Sqrt));
    res.insert((STANDARD_CONTEXT_ID, 12), ObjectRef::new(math::Exp));
    res.insert((STANDARD_CONTEXT_ID, 13), ObjectRef::new(math::Ln));
    res.insert((STANDARD_CONTEXT_ID, 14), ObjectRef::new(math::Arcsin));
    res.insert((STANDARD_CONTEXT_ID, 15), ObjectRef::new(math::Arccos));
    res.insert((STANDARD_CONTEXT_ID, 16), ObjectRef::new(math::Arctan));
    res.insert(
        (STANDARD_CONTEXT_ID, 17),
        ObjectRef::new(special_funcs::Type),
    );
    res.insert(
        (STANDARD_CONTEXT_ID, 18),
        ObjectRef::new(conversion::String_),
    );
    res.insert(
        (STANDARD_CONTEXT_ID, 19),
        ObjectRef::new(special_funcs::Clone_),
    );
    res.insert((STANDARD_CONTEXT_ID, 20), ObjectRef::new(conversion::Bool));
    res.insert((STANDARD_CONTEXT_ID, 21), ObjectRef::new(conversion::Int));
    res.insert((STANDARD_CONTEXT_ID, 22), ObjectRef::new(conversion::Float));
    res.insert((STANDARD_CONTEXT_ID, 23), ObjectRef::new(conversion::Char));
    res.insert(
        (STANDARD_CONTEXT_ID, 24),
        ObjectRef::new(special_funcs::Hash),
    );
    res.insert(
        (STANDARD_CONTEXT_ID, 25),
        ObjectRef::new(special_funcs::Lock),
    );
    res.insert((STANDARD_CONTEXT_ID, 26), ObjectRef::new(conversion::List_));
    res.insert((STANDARD_CONTEXT_ID, 27), ObjectRef::new(conversion::Set_));
    res.insert((STANDARD_CONTEXT_ID, 28), ObjectRef::new(sh::Args));
    res.insert((STANDARD_CONTEXT_ID, 29), ObjectRef::new(sh::Which));
    res.insert((STANDARD_CONTEXT_ID, 30), ObjectRef::new(functional::MapFunc));
    res.insert((STANDARD_CONTEXT_ID, 31), ObjectRef::new(functional::FilterFunc));
    res.insert((STANDARD_CONTEXT_ID, 32), ObjectRef::new(special_funcs::Print));
    res.insert((STANDARD_CONTEXT_ID, 33), ObjectRef::new(special_funcs::Printr));
    res.insert((STANDARD_CONTEXT_ID, 34), ObjectRef::new(special_funcs::Println));
    res.insert((STANDARD_CONTEXT_ID, 35), ObjectRef::new(special_funcs::Eprint));
    res.insert((STANDARD_CONTEXT_ID, 36), ObjectRef::new(special_funcs::Eprintr));
    res.insert((STANDARD_CONTEXT_ID, 37), ObjectRef::new(special_funcs::Eprintln));
    res.insert((STANDARD_CONTEXT_ID, 38), ObjectRef::new(sh::Open));
    res.insert((STANDARD_CONTEXT_ID, 39), ObjectRef::new(sh::Exists));
    res.insert((STANDARD_CONTEXT_ID, 40), ObjectRef::new(sh::IsDirectory));
    res.insert((STANDARD_CONTEXT_ID, 41), ObjectRef::new(sh::Canonicalize));
    res.insert((STANDARD_CONTEXT_ID, 42), ObjectRef::new(conversion::Dict_));
    res.insert((STANDARD_CONTEXT_ID, 43), ObjectRef::new(sh::Hostname));
    res.insert((STANDARD_CONTEXT_ID, 44), ObjectRef::new(sh::Devicename));
    res.insert((STANDARD_CONTEXT_ID, 45), ObjectRef::new(sh::Realname));
    res.insert((STANDARD_CONTEXT_ID, 46), ObjectRef::new(sh::Username));
    res.insert((STANDARD_CONTEXT_ID, 47), ObjectRef::new(sh::Langs));
    res.insert((STANDARD_CONTEXT_ID, 48), ObjectRef::new(sh::DesktopEnv));
    res.insert((STANDARD_CONTEXT_ID, 49), ObjectRef::new(special_funcs::Assert));
    res.insert((STANDARD_CONTEXT_ID, 50), ObjectRef::new(special_funcs::Version));
    res.insert((STANDARD_CONTEXT_ID, 51), ObjectRef::new(special_funcs::Stale));
    res.insert((STANDARD_CONTEXT_ID, 52), ObjectRef::new(sh::ScriptPath));
    res.insert((STANDARD_CONTEXT_ID, 53), ObjectRef::new(math::Rand));
    res.insert((STANDARD_CONTEXT_ID, 54), ObjectRef::new(math::RandRange));
    res.insert((STANDARD_CONTEXT_ID, 55), ObjectRef::new(math::RandNormal));
    res.insert((STANDARD_CONTEXT_ID, 56), ObjectRef::new(special_funcs::Sleep));
    res.insert((STANDARD_CONTEXT_ID, 57), ObjectRef::new(sh::StripPathPrefix));
    res.insert((STANDARD_CONTEXT_ID, 58), ObjectRef::new(string::StripPrefix));
    res.insert((STANDARD_CONTEXT_ID, 59), ObjectRef::new(string::StripSuffix));
    res
}

pub(crate) trait HumanReadableRange {
    fn print_range(&self) -> String;
}

pub(crate) fn print_range<T: HumanReadableRange>(val: &T) -> String {
    val.print_range()
}

use std::ops;

// Probably could be more general using stepping, but only meant to be used on integers
impl<T: std::fmt::Display + PartialEq + From<u8> + Copy> HumanReadableRange for ops::Range<T> 
where T: ops::Sub<T, Output=T>
{
    fn print_range(&self) -> String {
        if self.start == self.end - T::from(1u8) {
            self.start.to_string()
        } else {
            format!("{}-{}", self.start, self.end - T::from(1u8)).to_string()
        }
    }
}

impl<T: std::fmt::Display + Copy> HumanReadableRange for ops::RangeFrom<T> {
    fn print_range(&self) -> String {
        format!("at least {}", self.start).to_string()
    }
}

impl<T: std::fmt::Display + PartialEq + Copy> HumanReadableRange for ops::RangeInclusive<T> {
    fn print_range(&self) -> String {
        if *self.start() == *self.end() {
            self.start().to_string()
        } else {
            format!("{}-{}", self.start(), self.end()).to_string()
        }
    }
}

impl<T: std::fmt::Display + Copy> HumanReadableRange for ops::RangeTo<T> {
    fn print_range(&self) -> String {
        format!("less than {}", self.end).to_string()
    }
}

impl<T: std::fmt::Display + Copy> HumanReadableRange for ops::RangeToInclusive<T> {
    fn print_range(&self) -> String {
        format!("at most {}", self.end).to_string()
    }
}



#[macro_export]
macro_rules! func_object_void {
    ($id:ident, $args_range:tt, $context:ident, $args:ident -> $call:block) => {
        pub struct $id;

        impl Object for ObjectCell<$id> {
            fn technetium_type_name(&self) -> String {
                "builtin-func".to_string()
            }

            fn call(&self, $args: &[ObjectRef], $context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
                if !$args_range.contains(&$args.len()) {
                    return Err(RuntimeError::type_error(format!("Incorrect number of arguments: expected {}, got {}", crate::standard::print_range(&$args_range), $args.len())));
                }
                $call
                Ok(UnitObject::new())
            }
        }
    };
}

#[macro_export]
macro_rules! func_object {
    ($id:ident, $args_range:tt, $context:ident, $args:ident -> $call:block) => {
        pub struct $id;

        impl Object for ObjectCell<$id> {
            fn technetium_type_name(&self) -> String {
                "builtin-func".to_string()
            }

            fn call(
                &self,
                $args: &[ObjectRef],
                $context: &mut RuntimeContext<'_>,
            ) -> RuntimeResult<ObjectRef> {
                if !$args_range.contains(&$args.len()) {
                    return Err(RuntimeError::type_error(format!(
                        "Incorrect number of arguments: expected {}, got {}",
                        crate::standard::print_range(&$args_range),
                        $args.len()
                    )));
                }
                $call
            }
        }
    };
}
