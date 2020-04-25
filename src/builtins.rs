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

pub fn negate(a: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    match a_any.type_id() {
        a if a == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(-int_a.val);
            Ok(res)
        },
        a if a == TypeId::of::<FloatObject>() => {
            let int_a = a_any.downcast_ref::<FloatObject>().unwrap();
            let res = FloatObject::new(-int_a.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot negate type {}", a.marsh_type_name())))
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

pub fn index(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<List>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<List>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            if int_b.val < 0 {
                return Err(RuntimeError::index_oob_error("Negative index".to_string()));
            }
            if (int_b.val as u64 as usize) >= int_a.contents.len() {
                return Err(RuntimeError::index_oob_error("Index out of bounds".to_string()));
            }
            let res = Arc::clone(&int_a.contents[int_b.val as u64 as usize]);
            Ok(res)
        },
        (a, b) if a == TypeId::of::<String>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<String>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            if int_b.val < 0 {
                return Err(RuntimeError::index_oob_error("Negative index".to_string()));
            }
            let c = int_a.chars().nth(int_b.val as u64 as usize);
            if let Some(c) = c {
                let s = format!("{}", c);
                Ok(Arc::new(s))
            } else {
                Err(RuntimeError::index_oob_error(format!("Index out of bounds")))
            }
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot index type {} with type {}", a.marsh_type_name(), b.marsh_type_name())))
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
        let d = Arc::new("Hello".to_string());
        assert!(add(a, b).is_ok());
        assert!(add(c, d).is_ok());
    }
}

