use crate::builtins::*;
use crate::bytecode::NonLocalName;
use crate::bytecode::{ContextId, FrameId};
use crate::core::*;
use crate::error::*;
use std::collections::HashMap;
use parking_lot::RwLock;
use std::rc::Rc;

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

func_object!(Exit, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        exit(int_obj.to_i64()? as i32)
    } else {
        exit(if args[0].truthy() { 1 } else { 0 })
    }
});

func_object!(Type, (1..=1), args -> {
    Ok(StringObject::new(args[0].technetium_type_name()))
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

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(RangeIterator::new(this.clone()))
    }
}

pub struct RangeIterator {
    inner: Range,
    curr: RwLock<i64>,
}

impl RangeIterator {
    pub fn new(inner: Range) -> ObjectRef {
        ObjectRef::new(RangeIterator {
            curr: RwLock::new(inner.start),
            inner,
        })
    }
}

impl Object for ObjectCell<RangeIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(range)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let this = self.try_borrow()?;
        let mut _curr = this.curr.write();
        if (this.inner.step < 0 && *_curr <= this.inner.end)
            || (this.inner.step > 0 && *_curr >= this.inner.end)
        {
            return Ok(None);
        }
        let old = *_curr;
        *_curr += this.inner.step;
        Ok(Some(IntObject::new(old)))
    }
}

func_object!(RangeFunc, (1..=3), args -> {
    if args.len() == 1 {
        if let Some(int_obj) = args[0].as_any().downcast_ref::<IntObject>() {
            Ok(ObjectRef::new(Range { 
                start: 0,
                end: int_obj.to_i64()?,
                step: 1,
            }))
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range"))
        }
    } else if args.len() == 2 {
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<IntObject>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<IntObject>() {
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
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<IntObject>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<IntObject>() {
                if let Some(int_obj_c) = args[2].as_any().downcast_ref::<IntObject>() {
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
