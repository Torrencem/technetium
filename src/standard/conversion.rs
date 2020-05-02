
use crate::builtins::*;
use crate::bytecode::NonLocalName;
use crate::bytecode::{ContextId, FrameId};
use crate::core::*;
use crate::error::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::any::TypeId;

use std::io::{self, Write};
use std::process::{Child, Command, Output, Stdio};

use crate::func_object;

func_object!(String_, (1..=1), args -> {
    Ok(StringObject::new(args[0].to_string()?))
});

func_object!(Bool, (1..=1), args -> {
    Ok(BoolObject::new(args[0].truthy()))
});

pub fn to_int(val: ObjectRef) -> RuntimeResult<i64> {
    let val_any = val.as_any();
    match val_any.type_id() {
        a if a == TypeId::of::<IntObject>() => {
            Ok(val_any.downcast_ref::<IntObject>().unwrap().val)
        },
        a if a == TypeId::of::<FloatObject>() => {
            Ok(val_any.downcast_ref::<FloatObject>().unwrap().val as i64)
        },
        a if a == TypeId::of::<StringObject>() => {
            let as_str = val_any.downcast_ref::<StringObject>()
                .unwrap()
                .val
                .lock()
                .unwrap();

            Ok(as_str.parse::<i64>().map_err(|e| {
                RuntimeError::type_error(format!("Error converting string to int: {}", e.to_string()))
            })?)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Unable to convert from {} to int", val.technetium_type_name())))
        }
    }
}

func_object!(Int, (1..=1), args -> {
    Ok(IntObject::new(to_int(Arc::clone(&args[0]))?))
});

pub fn to_float(val: ObjectRef) -> RuntimeResult<f64> {
    let val_any = val.as_any();
    match val_any.type_id() {
        a if a == TypeId::of::<FloatObject>() => {
            Ok(val_any.downcast_ref::<FloatObject>().unwrap().val)
        },
        a if a == TypeId::of::<IntObject>() => {
            Ok(val_any.downcast_ref::<IntObject>().unwrap().val as f64)
        },
        a if a == TypeId::of::<StringObject>() => {
            let as_str = val_any.downcast_ref::<StringObject>()
                .unwrap()
                .val
                .lock()
                .unwrap();

            Ok(as_str.parse::<f64>().map_err(|e| {
                RuntimeError::type_error(format!("Error converting string to int: {}", e.to_string()))
            })?)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Unable to convert from {} to int", val.technetium_type_name())))
        }
    }
}

func_object!(Float, (1..=1), args -> {
    Ok(FloatObject::new(to_float(Arc::clone(&args[0]))?))
});
