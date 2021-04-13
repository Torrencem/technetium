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
use std::convert::TryInto;

use crate::standard::special_funcs::Range;

pub fn add(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    // Special case: if one of the objects is a string
    downcast!((a_str: StringObject = a) -> {
            let res = format!("{}{}", a_str.val, b.to_string()?);
            return Ok(StringObject::new(res));
    });
    downcast!((b_str: StringObject = b) -> {
            let res = format!("{}{}", a.to_string()?, b_str.val);
            return Ok(StringObject::new(res));
    });
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(val_a.val.clone() + val_b.val.clone());
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = FloatObject::new((val_a.to_i64()? as f64) + val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = FloatObject::new(val_a.val + (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = FloatObject::new(val_a.val + val_b.val);
            Ok(res)
        },
        (val_a: CharObject, val_b: CharObject) => {
            let res = format!("{}{}", val_a.val, val_b.val);
            Ok(StringObject::new(res))
        },
        (val_a: List, val_b: List) => {
            let mut res = val_a.contents.clone();
            res.append(&mut val_b.contents.clone());
            Ok(ObjectRef::new(List { contents: res }))
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot add type {} to type {}",
                a.technetium_type_name(),
                b.technetium_type_name())))
        }
    })
}

pub fn sub(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(val_a.val.clone() - val_b.val.clone());
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = FloatObject::new((val_a.to_i64()? as f64) - val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = FloatObject::new(val_a.val - (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = FloatObject::new(val_a.val - val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot subtract type {} to type {}",
                a.technetium_type_name(),
                b.technetium_type_name())))
        }
    })
}

pub fn mul(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(val_a.val.clone() * val_b.val.clone());
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = FloatObject::new((val_a.to_i64()? as f64) * val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = FloatObject::new(val_a.val * (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = FloatObject::new(val_a.val * val_b.val);
            Ok(res)
        },
        (val_a: List, val_b: IntObject) => {
            let mut res: Vec<ObjectRef> = Vec::with_capacity(val_b.to_i64()?.try_into().unwrap_or(0usize) * val_a.contents.len());
            for _ in 0..val_b.to_i64()? {
                for obj_ref in val_a.contents.iter() {
                    res.push(ObjectRef::clone(obj_ref));
                }
            }
            Ok(ObjectRef::new(List { contents: res }))
        },
        (val_a: IntObject, val_b: List) => {
            let mut res: Vec<ObjectRef> = Vec::with_capacity(val_a.to_i64()?.try_into().unwrap_or(0usize) * val_b.contents.len());
            for _ in 0..val_a.to_i64()? {
                for obj_ref in val_b.contents.iter() {
                    res.push(ObjectRef::clone(obj_ref));
                }
            }
            Ok(ObjectRef::new(List { contents: res }))
        },
        (val_a: StringObject, val_b: IntObject) => {
            let mut res = String::with_capacity(val_b.to_i64()?.try_into().unwrap_or(0usize) * val_a.val.len());
            for _ in 0..val_b.to_i64()? {
                res.push_str(&val_a.val);
            }
            Ok(ObjectRef::new(StringObject { val: res }))
        },
        (val_a: IntObject, val_b: StringObject) => {
            let mut res = String::with_capacity(val_a.to_i64()?.try_into().unwrap_or(0usize) * val_b.val.len());
            for _ in 0..val_a.to_i64()? {
                res.push_str(&val_b.val);
            }
            Ok(ObjectRef::new(StringObject { val: res }))
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot multiply type {} by type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn negate(a: ObjectRef) -> RuntimeResult<ObjectRef> {
    downcast!((val_a: IntObject = a) -> {
        let res = IntObject::new_big(-val_a.val.clone());
        return Ok(res);
    });
    downcast!((val_a: FloatObject = a) -> {
        let res = FloatObject::new(-val_a.val);
        return Ok(res);
    });
    Err(RuntimeError::type_error(format!(
        "Cannot negate type {}",
        a.technetium_type_name()
    )))
}

pub fn div(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(val_a.val.clone() / val_b.val.clone());
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = FloatObject::new((val_a.to_i64()? as f64) / val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = FloatObject::new(val_a.val / (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = FloatObject::new(val_a.val / val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot divide type {} by type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn mod_(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(val_a.val.modpow(&BigInt::one(), &val_b.val));
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = FloatObject::new((val_a.to_i64()? as f64).rem_euclid(val_b.val));
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = FloatObject::new(val_a.val.rem_euclid(val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = FloatObject::new(val_a.val.rem_euclid(val_b.val));
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot mod type {} by type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn not(a: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(if a.truthy() {
        BoolObject::new(false)
    } else {
        BoolObject::new(true)
    })
}

pub fn and(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(if a.truthy() && b.truthy() {
        BoolObject::new(true)
    } else {
        BoolObject::new(false)
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
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(&val_a.val & &val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot bitand type {} by type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn bitor(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(&val_a.val | &val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot bitor type {} by type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn bitxor(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = IntObject::new_big(&val_a.val ^ &val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot bitxor type {} by type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn cmp_lt(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = BoolObject::new((val_a.to_i64()? as f64) < val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val < (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        },
        (val_a: CharObject, val_b: CharObject) => {
            let res = BoolObject::new(val_a.val < val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot compare type {} and type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn cmp_gt(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = BoolObject::new((val_a.to_i64()? as f64) > val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val > (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        },
        (val_a: CharObject, val_b: CharObject) => {
            let res = BoolObject::new(val_a.val > val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot compare type {} and type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn cmp_leq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = BoolObject::new((val_a.to_i64()? as f64) <= val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val <= (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        },
        (val_a: CharObject, val_b: CharObject) => {
            let res = BoolObject::new(val_a.val <= val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot compare type {} and type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn cmp_geq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    match_tech_types!((a, b) {
        (val_a: IntObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        },
        (val_a: IntObject, val_b: FloatObject) => {
            let res = BoolObject::new((val_a.to_i64()? as f64) >= val_b.val);
            Ok(res)
        },
        (val_a: FloatObject, val_b: IntObject) => {
            let res = BoolObject::new(val_a.val >= (val_b.to_i64()? as f64));
            Ok(res)
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        },
        (val_a: CharObject, val_b: CharObject) => {
            let res = BoolObject::new(val_a.val >= val_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot compare type {} and type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

pub fn cmp_eq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(BoolObject::new(a == b))
}

pub fn cmp_neq(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    Ok(BoolObject::new(a != b))
}

pub fn index_get(a: ObjectRef, b: ObjectRef) -> RuntimeResult<ObjectRef> {
    downcast!((val_a: Dictionary = a) -> {
        let hashable = b.hashable().ok_or_else(|| {
            RuntimeError::type_error(format!(
                "Type {} used as a key in dictionary is not hashable",
                b.technetium_type_name()
            ))
        })?;
        return match val_a.contents.get(&hashable) {
            Some(res) => Ok(ObjectRef::clone(res)),
            None => Err(RuntimeError::key_error(
                "Read key from dictionary that doesn't exist",
            )),
        };
    });
    match_tech_types!((a, b) {
        (val_a: List, val_b: IntObject) => {
            let val_a = &val_a.contents;
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
        },
        (val_a: Tuple, val_b: IntObject) => {
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
        },
        (val_a: StringObject, val_b: IntObject) => {
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
        },
        (val_a: Slice, val_b: IntObject) => {
            let val_b = val_b.val.clone();
            let index = val_a.start + val_b * val_a.step;
            index_get(ObjectRef::clone(&val_a.parent), IntObject::new_big(index))
        },
        (val_a: Range, val_b: IntObject) => {
            let val_b = val_b.val.clone();
            let index = val_a.start + val_b * val_a.step;
            if index < val_a.start.into() || index >= val_a.end.into() {
                Err(RuntimeError::index_oob_error("Index accessed in range() is out of bounds"))
            } else {
                Ok(IntObject::new_big(index))
            }
        },
        _ => {
            Err(RuntimeError::type_error(format!(
                "Cannot index type {} with type {}",
                a.technetium_type_name(),
                b.technetium_type_name()
            )))
        }
    })
}

// This one is layed out without the helper macros for now because of mutability issues.
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
            hashable.lock_immutable();
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
