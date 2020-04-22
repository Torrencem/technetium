use crate::core::*;
use std::any::TypeId;
use std::sync::Arc;

pub fn add(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(int_a.val + int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((int_a.val as f64) + int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(int_a.val + (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(int_a.val + int_b.val);
            Ok(res)
        },
        (a, _) if a == TypeId::of::<String>() => {
            let a = a_any.downcast_ref::<String>().unwrap();
            let res = format!("{}{}", a, b.to_string()?);
            Ok(Arc::new(res))
        },
        (_, b) if b == TypeId::of::<String>() => {
            let b = b_any.downcast_ref::<String>().unwrap();
            let res = format!("{}{}", a.to_string()?, b);
            Ok(Arc::new(res))
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn sub(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(int_a.val - int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((int_a.val as f64) - int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(int_a.val - (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(int_a.val - int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn mul(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(int_a.val * int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((int_a.val as f64) * int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(int_a.val * (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(int_a.val * int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn div(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(int_a.val / int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new((int_a.val as f64) / int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = FloatObject::new(int_a.val / (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(int_a.val / int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn not(a: ObjectRef) -> Result<ObjectRef> {
    Ok(if a.truthy() {
        BoolObject::new(false)
    } else {
        BoolObject::new(true)
    })
}

pub fn or(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    Ok(if a.truthy() || b.truthy() {
        BoolObject::new(true)
    } else {
        BoolObject::new(false)
    })
}

pub fn and(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    Ok(if a.truthy() && b.truthy() {
        BoolObject::new(true)
    } else {
        BoolObject::new(false)
    })
}

pub fn cmp_lt(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val < int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((int_a.val as f64) < int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val < (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(int_a.val < int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn cmp_gt(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val > int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((int_a.val as f64) > int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val > (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(int_a.val > int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn cmp_eq(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val == int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((int_a.val as f64) == int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val == (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(int_a.val == int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn cmp_neq(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val != int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((int_a.val as f64) != int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val != (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(int_a.val != int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn cmp_leq(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val <= int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((int_a.val as f64) <= int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val <= (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(int_a.val <= int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

pub fn cmp_geq(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val >= int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new((int_a.val as f64) >= int_b.val);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = BoolObject::new(int_a.val >= (int_b.val as f64));
            Ok(res)
        },
        (a, b) if a == TypeId::of::<FloatObject>() && b == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let int_b = b_any.downcast_ref::<FloatObject>().unwrap();
            let res = BoolObject::new(int_a.val >= int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.marsh_type_name(), b.marsh_type_name())))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let a = IntObject::new(5);
        let b = IntObject::new(10);
        assert!(add(a, b).is_ok());
    }
}

