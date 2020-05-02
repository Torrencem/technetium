use crate::core::*;
use std::any::TypeId;
use std::sync::Arc;
use crate::error::*;

pub fn add(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(val_a.val + val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) + val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val + (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val + val_b.val);
            Ok(res)
        },
        (a, _) if a == TypeId::of::<StringObject>() => {
            let a = a_any.downcast_ref::<StringObject>().unwrap();
            let res = format!("{}{}", a.val.lock().unwrap(), b.to_string()?);
            Ok(StringObject::new(res))
        },
        (_, b) if b == TypeId::of::<StringObject>() => {
            let b = b_any.downcast_ref::<StringObject>().unwrap();
            let res = format!("{}{}", a.to_string()?, b.val.lock().unwrap());
            Ok(StringObject::new(res))
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) - val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val - (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val - val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot subtract type {} and type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) * val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val * (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val * val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot multiply type {} by type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
    }
}

pub fn negate(a: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    match a_any.type_id() {
        a if a == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(-val_a.val);
            Ok(res)
        },
        a if a == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(-val_a.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot negate type {}", a.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) / val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val / (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val / val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot divide type {} by type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((val_a.val as f64) % val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(val_a.val % (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(val_a.val % val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot mod type {} by type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) < val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val < (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot compare type {} with type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) > val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val > (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot compare type {} with type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) == val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val == (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val == val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val == val_b.val);
            Ok(res)
        },
        (a_, b_) if a_ == TypeId::of::<StringObject>() && b_ == TypeId::of::<StringObject>() => {
            let val_a = a_any.downcast_ref::<StringObject>().unwrap();
            let val_b = b_any.downcast_ref::<StringObject>().unwrap();
            // Check for alias to avoid deadlock
            if Arc::ptr_eq(&a, &b) {
                Ok(BoolObject::new(true))   
            } else {
                let res = BoolObject::new(*val_a.val.lock().unwrap() == *val_b.val.lock().unwrap());
                Ok(res)
            }
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot equate type {} to type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) != val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val != (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val != val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val != val_b.val);
            Ok(res)
        },
        (a_, b_) if a_ == TypeId::of::<StringObject>() && b_ == TypeId::of::<StringObject>() => {
            let val_a = a_any.downcast_ref::<StringObject>().unwrap();
            let val_b = b_any.downcast_ref::<StringObject>().unwrap();
            // Check for alias to avoid deadlock
            if Arc::ptr_eq(&a, &b) {
                Ok(BoolObject::new(false))   
            } else {
                let res = BoolObject::new(*val_a.val.lock().unwrap() != *val_b.val.lock().unwrap());
                Ok(res)
            }
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot equate type {} to type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) <= val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val <= (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot compare type {} with type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<IntObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((val_a.val as f64) >= val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(val_a.val >= (val_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let val_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let val_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<CharObject>() && b == TypeId::of::<CharObject>() => {
            let val_a = a_any.downcast_ref::<CharObject>().unwrap();
            let val_b = b_any.downcast_ref::<CharObject>().unwrap();
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot compare type {} with type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
    }
}

pub fn index_get(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<List>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<List>().unwrap().contents.lock().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            if val_b.val < 0 {
                return Err(RuntimeError::index_oob_error("Negative index"));
            }
            if (val_b.val as u64 as usize) >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            let res = Arc::clone(&val_a[val_b.val as u64 as usize]);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<Tuple>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<Tuple>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            if val_b.val < 0 {
                return Err(RuntimeError::index_oob_error("Negative index"));
            }
            if (val_b.val as u64 as usize) >= val_a.contents.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            let res = Arc::clone(&val_a.contents[val_b.val as u64 as usize]);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<StringObject>() && b == TypeId::of::<IntObject>() => {
            let val_a = a_any.downcast_ref::<StringObject>().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            if val_b.val < 0 {
                return Err(RuntimeError::index_oob_error("Negative index"));
            }
            let c = val_a.val.lock().unwrap().chars().nth(val_b.val as u64 as usize);
            if let Some(c) = c {
                Ok(CharObject::new(c))
            } else {
                Err(RuntimeError::index_oob_error(format!("Index out of bounds")))
            }
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot index type {} with type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
    }
}

pub fn index_set(a: ObjectRef, b: ObjectRef, c: ObjectRef) -> RuntimeResult<()> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<List>() && b == TypeId::of::<IntObject>() => {
            let mut val_a = a_any.downcast_ref::<List>().unwrap().contents.lock().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            if val_b.val < 0 {
                return Err(RuntimeError::index_oob_error("Negative index"));
            }
            if (val_b.val as u64 as usize) >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            val_a[val_b.val as u64 as usize] = c;
            Ok(())
        },
        (a, b) if a == TypeId::of::<StringObject>() && b == TypeId::of::<IntObject>() && c.as_any().is::<CharObject>() => {
            let mut val_a = a_any.downcast_ref::<StringObject>().unwrap().val.lock().unwrap();
            let val_b = b_any.downcast_ref::<IntObject>().unwrap();
            let index = val_b.val as u64 as usize;
            let val_c = c.as_any().downcast_ref::<CharObject>().unwrap();
            let ch = val_c.val;
            if val_b.val < 0 {
                return Err(RuntimeError::index_oob_error("Negative index"));
            }
            if (val_b.val as u64 as usize) >= val_a.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds"));
            }
            val_a.replace_range(index..index+1, &ch.to_string());
            Ok(())
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot index type {} with type {}", a.technetium_type_name(), b.technetium_type_name())))
        },
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

