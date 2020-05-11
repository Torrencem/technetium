
use crate::builtins::*;
use crate::bytecode::NonLocalName;
use crate::bytecode::{ContextId, FrameId};
use crate::core::*;
use crate::error::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::rc::Rc;
use std::any::TypeId;
use std::char;
use std::u32;

use std::io::{self, Write};
use std::process::{Child, Command, Output, Stdio};

use num::BigInt;
use num::bigint::ToBigInt;

use crate::func_object;

func_object!(String_, (1..=1), args -> {
    Ok(StringObject::new(args[0].try_borrow()?.to_string()?))
});

func_object!(Bool, (1..=1), args -> {
    Ok(BoolObject::new(args[0].try_borrow()?.truthy()))
});

pub fn to_int(val: ObjectRef) -> RuntimeResult<BigInt> {
    let val = val.try_borrow()?;
    let val_any = val.as_any();
    match val_any.type_id() {
        a if a == TypeId::of::<IntObject>() => {
            Ok(val_any.downcast_ref::<IntObject>().unwrap().val.clone())
        },
        a if a == TypeId::of::<FloatObject>() => {
            Ok((val_any.downcast_ref::<FloatObject>().unwrap().val as i64).to_bigint().unwrap())
        },
        a if a == TypeId::of::<StringObject>() => {
            let as_str = val_any.downcast_ref::<StringObject>()
                .unwrap()
                .val
                .read();

            Ok(as_str.parse::<BigInt>().map_err(|e| {
                RuntimeError::type_error(format!("Error converting string to int: {}", e.to_string()))
            })?)
        },
        a if a == TypeId::of::<CharObject>() => {
            Ok((val_any.downcast_ref::<CharObject>().unwrap().val as u32).to_bigint().unwrap())
        },
        _ => {
            Err(RuntimeError::type_error(format!("Unable to convert from {} to int", val.technetium_type_name())))
        }
    }
}

func_object!(Int, (1..=1), args -> {
    Ok(IntObject::new_big(to_int(ObjectRef::clone(&args[0]))?))
});

pub fn to_float(val: ObjectRef) -> RuntimeResult<f64> {
    let val = val.try_borrow()?;
    let val_any = val.as_any();
    match val_any.type_id() {
        a if a == TypeId::of::<FloatObject>() => {
            Ok(val_any.downcast_ref::<FloatObject>().unwrap().val)
        },
        a if a == TypeId::of::<IntObject>() => {
            Ok(val_any.downcast_ref::<IntObject>().unwrap().to_i64()? as f64)
        },
        a if a == TypeId::of::<StringObject>() => {
            let as_str = val_any.downcast_ref::<StringObject>()
                .unwrap()
                .val
                .read();

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
    Ok(FloatObject::new(to_float(ObjectRef::clone(&args[0]))?))
});

pub fn to_char(val: ObjectRef) -> RuntimeResult<char> {
    let val = val.try_borrow()?;
    let val_any = val.as_any();
    match val_any.type_id() {
        a if a == TypeId::of::<IntObject>() => {
            let as_int = val_any.downcast_ref::<IntObject>().unwrap().to_i64()?;
            if as_int < 0 || as_int > u32::MAX as i64 {
                Err(RuntimeError::type_error("Value out of range to be converted to character"))
            } else {
                let as_char = char::from_u32(as_int as u32);
                if let Some(c) = as_char {
                    Ok(c)
                } else {
                    Err(RuntimeError::type_error(format!("Integer does not map to character: {}", as_int)))
                }
            }
        },
        a if a == TypeId::of::<StringObject>() => {
            let as_str = val_any.downcast_ref::<StringObject>()
                .unwrap()
                .val
                .read();
            
            if as_str.len() != 1 {
                Err(RuntimeError::type_error(format!("Unable to convert string of length {} to character", as_str.len())))
            } else {
                Ok(as_str.chars().nth(0).unwrap())
            }
        },
        _ => {
            Err(RuntimeError::type_error(format!("Unable to convert from {} to int", val.technetium_type_name())))
        }
    }
}

func_object!(Char, (1..=1), args -> {
    Ok(CharObject::new(to_char(ObjectRef::clone(&args[0]))?))
});
