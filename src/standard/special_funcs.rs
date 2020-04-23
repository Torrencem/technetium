
use crate::core::*;
use crate::builtins::*;

use crate::{func_object, func_object_void};

use std::process::exit;

func_object_void!(Print, (0..), args -> {
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
});

func_object!(Exit, (1..1), args -> {
    let arg_any = args[0].as_any();
    if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        exit(int_obj.val as i32)
    } else {
        exit(if args[0].truthy() { 1 } else { 0 })
    }
});
