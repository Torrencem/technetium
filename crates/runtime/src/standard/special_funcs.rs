use crate::*;
use crate::error::*;

use crate::{func_object, func_object_void};

use std::process::exit;

use num::bigint::ToBigInt;

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

func_object!(Exit, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        exit(int_obj.to_i64()? as i32)
    } else {
        exit(if args[0].truthy() { 1 } else { 0 })
    }
});

func_object!(Type, (1..=1), args -> {
    Ok(StringObject::new(args[0].technetium_type_name()))
});

func_object!(Hash, (1..=1), args -> {
    let hash = args[0].technetium_hash().ok_or_else(|| RuntimeError::type_error(format!("Unhashable type: {}", args[0].technetium_type_name())))?;
    let hash = hash.to_bigint().unwrap();
    Ok(IntObject::new_big(hash))
});

func_object_void!(Lock, (1..=1), args -> {
    args[0].lock_immutable()
});

func_object!(Clone_, (1..=1), args -> {
    Ok(args[0].technetium_clone()?)
});

#[derive(Debug, Clone)]
pub struct Range {
    start: i64,
    end: i64,
    step: i64,
}

impl Object for ObjectCell<Range> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(this.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "range".to_string()
    }

    fn lock_immutable(&self) {
        self.lock()
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(RangeIterator::new(this.clone()))
    }
}

pub struct RangeIterator {
    inner: Range,
    curr: i64,
}

impl RangeIterator {
    pub fn new(inner: Range) -> ObjectRef {
        ObjectRef::new(RangeIterator {
            curr: inner.start,
            inner,
        })
    }
}

impl Object for ObjectCell<RangeIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(range)".to_string()
    }

    fn lock_immutable(&self) {
        self.lock()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        let step = this.inner.step;
        let end = this.inner.end;
        let _curr = &mut this.curr;
        if (step < 0 && *_curr <= end)
            || (step > 0 && *_curr >= end)
        {
            return Ok(None);
        }
        let old = *_curr;
        *_curr += step;
        Ok(Some(IntObject::new(old)))
    }
}

func_object!(RangeFunc, (1..=3), args -> {
    if args.len() == 1 {
        if let Some(int_obj) = args[0].as_any().downcast_ref::<ObjectCell<IntObject>>() {
            let int_obj = int_obj.try_borrow()?;
            Ok(ObjectRef::new(Range { 
                start: 0,
                end: int_obj.to_i64()?,
                step: 1,
            }))
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range"))
        }
    } else if args.len() == 2 {
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<ObjectCell<IntObject>>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<ObjectCell<IntObject>>() {
                let int_obj_a = int_obj_a.try_borrow()?;
                let int_obj_b = int_obj_b.try_borrow()?;
                Ok(ObjectRef::new(Range {
                    start: int_obj_a.to_i64()?,
                    end: int_obj_b.to_i64()?,
                    step: 1,
                }))
            } else {
                Err(RuntimeError::type_error("Expected integer arguments to range"))
            }
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range"))
        }
    } else {
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<ObjectCell<IntObject>>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<ObjectCell<IntObject>>() {
                if let Some(int_obj_c) = args[2].as_any().downcast_ref::<ObjectCell<IntObject>>() {
                    let int_obj_a = int_obj_a.try_borrow()?;
                    let int_obj_b = int_obj_b.try_borrow()?;
                    let int_obj_c = int_obj_c.try_borrow()?;
                    Ok(ObjectRef::new(Range {
                        start: int_obj_a.to_i64()?,
                        end: int_obj_b.to_i64()?,
                        step: int_obj_c.to_i64()?,
                    }))
                } else {
                    Err(RuntimeError::type_error("Expected integer arguments to range"))
                }
            } else {
                Err(RuntimeError::type_error("Expected integer arguments to range"))
            }
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range"))
        }
    }
});
