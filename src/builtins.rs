use crate::core::*;
use crate::error::*;
use std::any::TypeId;
use std::sync::RwLock;
use std::rc::Rc;

pub fn add(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(val_a.val + val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) + val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val + (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val + val_b.val);
            Ok(res)
        }
        (a, _) if a == TypeId::of::<StringObject>() => {
            let a = a_any.downcast_ref::<StringObject>().unwrap();
            let res = format!("{}{}", a.val.read()?, b.to_string()?);
            Ok(StringObject::new(res))
        }
        (_, b) if b == TypeId::of::<StringObject>() => {
            let b = b_any.downcast_ref::<StringObject>().unwrap();
            let res = format!("{}{}", a.to_string()?, b.val.read()?);
            Ok(StringObject::new(res))
        }
        (a_, b_) if a_ == TypeId::of::<List>() && b_ == TypeId::of::<List>() => {
            let val_a = a_any.downcast_ref::<List>().unwrap();
            let val_b = b_any.downcast_ref::<List>().unwrap();
            if Rc::ptr_eq(&a, &b) {
                let mut res = val_a.contents.read()?.clone();
                res.append(&mut val_a.contents.read()?.clone());
                Ok(Rc::new(List { contents: RwLock::new(res) }))
            } else {
                let mut res = val_a.contents.read()?.clone();
                res.append(&mut val_b.contents.read()?.clone());
                Ok(Rc::new(List { contents: RwLock::new(res) }))
            }
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
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(val_a.val - val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) - val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val - (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
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
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(val_a.val * val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) * val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val * (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val * val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<List>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<List>().unwrap().contents.read()?;
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let mut res: Vec<ObjectRef> = vec![];
            for _ in (0..val_b.val) {
                for obj_ref in val_a.iter() {
                    res.push(Rc::clone(obj_ref));
                }
            }
            Ok(Rc::new(List { contents: RwLock::new(res) }))
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<List>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<List>().unwrap().contents.read()?;
            let mut res: Vec<ObjectRef> = vec![];
            for _ in (0..val_a.val) {
                for obj_ref in val_b.iter() {
                    res.push(Rc::clone(obj_ref));
                }
            }
            Ok(Rc::new(List { contents: RwLock::new(res) }))
        }
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
        a if a == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(-val_a.val);
            Ok(res)
        }
        a if a == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
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
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(val_a.val / val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) / val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val / (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
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
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(val_a.val % val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) % val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val % (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val % val_b.val);
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
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) < val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val < (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
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
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) > val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val > (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
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
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val == val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) == val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val == (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val == val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val == val_b.val);
            Ok(res)
        }
        (a_, b_) if a_ == TypeId::of::<StringObject>() && b_ == TypeId::of::<StringObject>() => {
            let val_a = a_any.downcast_ref::<StringObject>().unwrap();
            let val_b = b_any.downcast_ref::<StringObject>().unwrap();
            // Check for alias to avoid deadlock
            if Rc::ptr_eq(&a, &b) {
                Ok(BoolObject::new(true))
            } else {
                let res = BoolObject::new(*val_a.val.read()? == *val_b.val.read()?);
                Ok(res)
            }
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot equate type {} to type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn cmp_neq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val != val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) != val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val != (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val != val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val != val_b.val);
            Ok(res)
        }
        (a_, b_) if a_ == TypeId::of::<StringObject>() && b_ == TypeId::of::<StringObject>() => {
            let val_a = a_any.downcast_ref::<StringObject>().unwrap();
            let val_b = b_any.downcast_ref::<StringObject>().unwrap();
            // Check for alias to avoid deadlock
            if Rc::ptr_eq(&a, &b) {
                Ok(BoolObject::new(false))
            } else {
                let res = BoolObject::new(*val_a.val.read()? != *val_b.val.read()?);
                Ok(res)
            }
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot equate type {} to type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
        ))),
    }
}

pub fn cmp_leq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) <= val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val <= (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
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
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) >= val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val >= (val_b.val as f64));
            Ok(res)
        }
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
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
        (a, b) if a == TypeId::of::<List>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any
                .downcast_ref::<List>()
                .unwrap()
                .contents
                .read()?;
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let mut val = val_b.val;
            if val < 0 {
                val = (val_a.len() as u64 as i64) + val;
            }
            let val = val as u64 as usize;
            if val >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            let res = Rc::clone(&val_a[val]);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<Tuple>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<Tuple>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let mut val = val_b.val;
            if val < 0 {
                val = (val_a.contents.len() as u64 as i64) + val;
            }
            let val = val as u64 as usize;
            if (val as u64 as usize) >= val_a.contents.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            let res = Rc::clone(&val_a.contents[val]);
            Ok(res)
        }
        (a, b) if a == TypeId::of::<StringObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<StringObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let mut val = val_b.val;
            if val < 0 {
                val = (val_a.val.read()?.len() as u64 as i64) + val;
            }
            let val = val as u64 as usize;
            let c = val_a
                .val
                .read()?
                .chars()
                .nth(val);
            if let Some(c) = c {
                Ok(CharObject::new(c))
            } else {
                Err(RuntimeError::index_oob_error(format!(
                    "Index out of bounds"
                )))
            }
        }
        (a, b) if a == TypeId::of::<Slice>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<Slice>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap().val;
            let index = val_a.start + val_b * val_a.step;
            index_get(Rc::clone(&val_a.parent), IntObject::new(index))
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
        (a, b) if a == TypeId::of::<List>() && b == TypeId::of::<IntObject>() => {
            let mut val_a = a_any
                .downcast_ref::<List>()
                .unwrap()
                .contents
                .write()?;
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let mut val = val_b.val;
            if val < 0 {
                val = (val_a.len() as u64 as i64) + val;
            }
            let val = val as u64 as usize;
            if val >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            val_a[val] = c;
            Ok(())
        }
        (a, b)
            if a == TypeId::of::<StringObject>()
                && b == TypeId::of::<IntObject>()
                && c.as_any().is::<CharObject>() =>
        {
            let mut val_a = a_any
                .downcast_ref::<StringObject>()
                .unwrap()
                .val
                .write()?;
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let mut val = val_b.val;
            if val < 0 {
                val = (val_a.len() as u64 as i64) + val;
            }
            let index = val as u64 as usize;
            let val_c = c.as_any().downcast_ref::<CharObject>().unwrap();
            let ch = val_c.val;
            if index >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            val_a.replace_range(index..index + 1, &ch.to_string());
            Ok(())
        }
        (a, b) if a == TypeId::of::<Slice>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<Slice>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap().val;
            let index = val_a.start + val_b * val_a.step;
            index_set(Rc::clone(&val_a.parent), IntObject::new(index), c)
        }
        _ => Err(RuntimeError::type_error(format!(
            "Cannot index type {} with type {}",
            a.technetium_type_name(),
            b.technetium_type_name()
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
