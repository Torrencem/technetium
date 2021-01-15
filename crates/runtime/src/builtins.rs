//! Builtin functions. These are usually specially generated in the parse stage,
//! and compiled into special op codes. These operations are hard coded for the
//! builtin types.

use crate::error::*;
use crate::prelude::*;
use num::bigint::BigInt;
use num::traits::identities::One;
use num::traits::identities::Zero;
use num::traits::ToPrimitive;
use std::any::TypeId;

pub fn add(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(val_a.val.clone() + val_b.val.clone());
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new((val_a.to_i64()? as f64) + val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val + (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val + val_b.val);
            Ok(res)
        }
        (a, _) if a == TypeId::of::<ObjectCell<StringObject>>() => {
            let a = a_any
                .downcast_ref::<ObjectCell<StringObject>>()
                .unwrap()
                .try_borrow()?;
            let res = format!("{}{}", a.val, b.to_string()?);
            Ok(StringObject::new(res))
        }
        (_, b) if b == TypeId::of::<ObjectCell<StringObject>>() => {
            let b = b_any
                .downcast_ref::<ObjectCell<StringObject>>()
                .unwrap()
                .try_borrow()?;
            let res = format!("{}{}", a.to_string()?, b.val);
            Ok(StringObject::new(res))
        }
        (a_, b_)
            if a_ == TypeId::of::<ObjectCell<List>>() && b_ == TypeId::of::<ObjectCell<List>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<List>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<List>>()
                .unwrap()
                .try_borrow()?;
            let mut res = val_a.contents.clone();
            res.append(&mut val_b.contents.clone());
            Ok(ObjectRef::new(List { contents: res }))
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot add type {} to type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn sub(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(val_a.val.clone() - val_b.val.clone());
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new((val_a.to_i64()? as f64) - val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val - (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val - val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot subtract type {} and type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn mul(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(val_a.val.clone() * val_b.val.clone());
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new((val_a.to_i64()? as f64) * val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val * (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val * val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<List>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<List>>()
                .unwrap()
                .try_borrow()?;
            let val_a = &val_a.contents;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let mut res: Vec<ObjectRef> = vec![];
            for _ in 0..val_b.to_i64()? {
                for obj_ref in val_a.iter() {
                    res.push(ObjectRef::clone(obj_ref));
                }
            }
            Ok(ObjectRef::new(List { contents: res }))
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<List>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<List>>()
                .unwrap()
                .try_borrow()?;
            let val_b = &val_b.contents;
            let mut res: Vec<ObjectRef> = vec![];
            for _ in 0..val_a.to_i64()? {
                for obj_ref in val_b.iter() {
                    res.push(ObjectRef::clone(obj_ref));
                }
            }
            Ok(ObjectRef::new(List { contents: res }))
        }
        // TODO: Add int * string and string * int
        _ => Err(RuntimeError::type_error(format!(
            "Cannot multiply type {} by type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn negate(a: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    match a_any.type_id() {
        a if a == TypeId::of::<ObjectCell<IntObject>>() => {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(-val_a.val.clone());
            Ok(res)
        }
        a if a == TypeId::of::<ObjectCell<FloatObject>>() => {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(-val_a.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot negate type {}",
            a.technetium_type_name()
        ))),
    }
}

pub fn div(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(val_a.val.clone() / val_b.val.clone());
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new((val_a.to_i64()? as f64) / val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val / (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val / val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot divide type {} by type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn mod_(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(val_a.val.modpow(&BigInt::one(), &val_b.val));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new((val_a.to_i64()? as f64).rem_euclid(val_b.val));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val.rem_euclid(val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = FloatObject::new(val_a.val.rem_euclid(val_b.val));
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot mod type {} by type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn not(a: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(if a.truthy() {
        BoolObject::new(false)
    } else {
        BoolObject::new(true)
    })
}

pub fn or(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(if a.truthy() || b.truthy() {
        BoolObject::new(true)
    } else {
        BoolObject::new(false)
    })
}

pub fn bitand(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(&val_a.val & &val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot bitand type {} by type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn bitor(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(&val_a.val | &val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot bitor type {} by type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn bitxor(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = IntObject::new_big(&val_a.val ^ &val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot bitxor type {} by type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn and(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(if a.truthy() && b.truthy() {
        BoolObject::new(true)
    } else {
        BoolObject::new(false)
    })
}

pub fn cmp_lt(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new((val_a.to_i64()? as f64) < val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val < (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<CharObject>>()
                && b == TypeId::of::<ObjectCell<CharObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot compare type {} with type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn cmp_gt(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new((val_a.to_i64()? as f64) > val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val > (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<CharObject>>()
                && b == TypeId::of::<ObjectCell<CharObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot compare type {} with type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn cmp_eq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(BoolObject::new(a == b))
}

pub fn cmp_neq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(BoolObject::new(a != b))
}

pub fn cmp_leq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new((val_a.to_i64()? as f64) <= val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val <= (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<CharObject>>()
                && b == TypeId::of::<ObjectCell<CharObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot compare type {} with type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn cmp_geq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<IntObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new((val_a.to_i64()? as f64) >= val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val >= (val_b.to_i64()? as f64));
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<FloatObject>>()
                && b == TypeId::of::<ObjectCell<FloatObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<FloatObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<CharObject>>()
                && b == TypeId::of::<ObjectCell<CharObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot compare type {} with type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn index_get(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<List>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<List>>()
                .unwrap()
                .try_borrow()?;

            let val_a = &val_a.contents;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let mut val = val_b.val.clone();
            if val < BigInt::zero() {
                val = (val_a.len() as u64 as i64) + val;
            }
            let val = val
                .to_usize()
                .ok_or_else(|| RuntimeError::index_oob_error("Index out of bounds"))?;
            if val >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            let res = ObjectRef::clone(&val_a[val]);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<Tuple>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<Tuple>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let mut val = val_b.val.clone();
            if val < BigInt::zero() {
                val = (val_a.contents.len() as u64 as i64) + val;
            }
            let val = val
                .to_usize()
                .ok_or_else(|| RuntimeError::index_oob_error("Index out of bounds"))?;
            if (val as u64 as usize) >= val_a.contents.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            let res = ObjectRef::clone(&val_a.contents[val]);
            Ok(res)
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<StringObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<StringObject>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let mut val = val_b.val.clone();
            if val < BigInt::zero() {
                val = (val_a.val.len() as u64 as i64) + val;
            }
            let val = val
                .to_usize()
                .ok_or_else(|| RuntimeError::index_oob_error("Index out of bounds"))?;
            let c = val_a.val.chars().nth(val);
            if let Some(c) = c {
                Ok(CharObject::new(c))
            } else {
                Err(RuntimeError::index_oob_error(
                    "Index out of bounds".to_string(),
                ))
            }
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<Slice>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<Slice>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?
                .val
                .clone();
            let index = val_a.start + val_b * val_a.step;
            index_get(ObjectRef::clone(&val_a.parent), IntObject::new_big(index))
        }
        (a, _) if a == TypeId::of::<ObjectCell<Dictionary>>() => {
            let val_a = a_any
                .downcast_ref::<ObjectCell<Dictionary>>()
                .unwrap()
                .try_borrow()?;
            let hashable = b.hashable().ok_or_else(|| {
                RuntimeError::type_error(format!(
                    "Type {} used as a key in dictionary is not hashable",
                    b.technetium_type_name()
                ))
            })?;
            match val_a.contents.get(&hashable) {
                Some(res) => Ok(ObjectRef::clone(res)),
                None => Err(RuntimeError::key_error(
                    "Read key from dictionary that doesn't exist",
                )),
            }
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot index type {} with type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn index_set(a: ObjectRef, b: ObjectRef, c: ObjectRef) -> RuntimeResult<()> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b)
            if a == TypeId::of::<ObjectCell<List>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let mut val_a = a_any
                .downcast_ref::<ObjectCell<List>>()
                .unwrap()
                .try_borrow_mut()?;

            let val_a = &mut val_a.contents;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let mut val = val_b.val.clone();
            if val < BigInt::zero() {
                val = (val_a.len() as u64 as i64) + val;
            }
            let val = val
                .to_usize()
                .ok_or_else(|| RuntimeError::index_oob_error("Index out of bounds"))?;
            if val >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            val_a[val] = c;
            Ok(())
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<StringObject>>()
                && b == TypeId::of::<ObjectCell<IntObject>>()
                && c.as_any().is::<ObjectCell<CharObject>>() =>
        {
            let mut val_a = a_any
                .downcast_ref::<ObjectCell<StringObject>>()
                .unwrap()
                .try_borrow_mut()?;

            let val_a = &mut val_a.val;

            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?;
            let mut val = val_b.val.clone();
            if val < BigInt::zero() {
                val = (val_a.len() as u64 as i64) + val;
            }
            let index = val
                .to_usize()
                .ok_or_else(|| RuntimeError::index_oob_error("Index out of bounds"))?;
            let val_c = c
                .as_any()
                .downcast_ref::<ObjectCell<CharObject>>()
                .unwrap()
                .try_borrow()?;
            let ch = val_c.val;
            if index >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            val_a.replace_range(index..index + 1, &ch.to_string());
            Ok(())
        }
        (a, b)
            if a == TypeId::of::<ObjectCell<Slice>>()
                && b == TypeId::of::<ObjectCell<IntObject>>() =>
        {
            let val_a = a_any
                .downcast_ref::<ObjectCell<Slice>>()
                .unwrap()
                .try_borrow()?;
            let val_b = b_any
                .downcast_ref::<ObjectCell<IntObject>>()
                .unwrap()
                .try_borrow()?
                .val
                .clone();
            let index = val_a.start + val_b * val_a.step;
            index_set(
                ObjectRef::clone(&val_a.parent),
                IntObject::new_big(index),
                c,
            )
        }
        (a, _) if a == TypeId::of::<ObjectCell<Dictionary>>() => {
            let mut val_a = a_any
                .downcast_ref::<ObjectCell<Dictionary>>()
                .unwrap()
                .try_borrow_mut()?;
            let hashable = b.hashable().ok_or_else(|| {
                RuntimeError::type_error(format!(
                    "Type {} used as a key in dictionary is not hashable",
                    b.technetium_type_name()
                ))
            })?;
            val_a.contents.insert(hashable, c);
            Ok(())
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot set a[b] = c, where a is of type {}, b is of type {}, and c is of type {}",
            a.technetium_type_name(),
            b.technetium_type_name(),
            c.technetium_type_name(),
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_dispatch() {
        let a = IntObject::new(5);
        let b = IntObject::new(10);
        let c = FloatObject::new(5.0);
        let d = StringObject::new("Hello".to_string());
        assert!(add(a, b).is_ok());
        assert!(add(c, d).is_ok());
    }
}
