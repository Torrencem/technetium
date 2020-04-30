
use crate::core::*;
use crate::builtins::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bytecode::NonLocalName;
use crate::bytecode::{ContextId, FrameId};
use crate::error::*;
use std::env;
use std::path::Path;

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
        exit(int_obj.val as i32)
    } else {
        exit(if args[0].truthy() { 1 } else { 0 })
    }
});

func_object!(Cd, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(str_obj) = arg_any.downcast_ref::<StringObject>() {
        let path = Path::new(&str_obj.val);
        env::set_current_dir(path)?;
        Ok(VoidObject::new())
    } else {
        Err(RuntimeError::type_error("Expected string as argument to cd".to_string()))
    }
});

#[derive(Debug, Clone)]
pub struct Range {
    start: i64,
    end: i64,
    step: i64,
}

impl Object for Range {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(self.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "range".to_string()
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        Ok(RangeIterator::new(self.clone()))
    }
}

pub struct RangeIterator {
    inner: Range,
    curr: Mutex<i64>,
}

impl RangeIterator {
    pub fn new(inner: Range) -> ObjectRef {
        Arc::new(RangeIterator {
            curr: Mutex::new(inner.start),
            inner,
        })
    }
}

impl Object for RangeIterator {
    fn technetium_type_name(&self) -> String {
        "iterator(range)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let mut _curr = self.curr.lock().unwrap();
        if (self.inner.step < 0 && *_curr <= self.inner.end)
        || (self.inner.step > 0 && *_curr >= self.inner.end) {
            return Ok(None);
        }
        let old = *_curr;
        *_curr += self.inner.step;
        Ok(Some(IntObject::new(old)))
    }
}

func_object!(RangeFunc, (1..=3), args -> {
    if args.len() == 1 {
        if let Some(int_obj) = args[0].as_any().downcast_ref::<IntObject>() {
            Ok(Arc::new(Range { 
                start: 0,
                end: int_obj.val,
                step: 1,
            }))
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range".to_string()))
        }
    } else if args.len() == 2 {
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<IntObject>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<IntObject>() {
                Ok(Arc::new(Range {
                    start: int_obj_a.val,
                    end: int_obj_b.val,
                    step: 1,
                }))
            } else {
                Err(RuntimeError::type_error("Expected integer arguments to range".to_string()))
            }
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range".to_string()))
        }
    } else {
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<IntObject>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<IntObject>() {
                if let Some(int_obj_c) = args[2].as_any().downcast_ref::<IntObject>() {
                    Ok(Arc::new(Range {
                        start: int_obj_a.val,
                        end: int_obj_b.val,
                        step: int_obj_c.val,
                    }))
                } else {
                    Err(RuntimeError::type_error("Expected integer arguments to range".to_string()))
                }
            } else {
                Err(RuntimeError::type_error("Expected integer arguments to range".to_string()))
            }
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range".to_string()))
        }
    }
});
