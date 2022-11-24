//! Core objects such as integers, booleans and strings

use crate::*;
use builtins::index_get;
use bytecode::Op;
use bytecode::{ContextId, FrameId};
use error::*;
use num::bigint::{BigInt, ToBigInt};
use num::cast::ToPrimitive;
use ouroboros::self_referencing;
use parking_lot::RwLock;
use pretty_dtoa::dtoa;
use std::any::TypeId;
use std::clone::Clone as RustClone;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Hash)]
pub struct BoolObject {
    pub val: bool,
}

impl BoolObject {
    pub fn new(val: bool) -> ObjectRef {
        ObjectRef::new(BoolObject { val })
    }
}

impl Object for ObjectCell<BoolObject> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(BoolObject::new(this.val))
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        self.try_borrow().ok()?.hash(&mut hasher);
        Some(hasher.finish())
    }

    fn technetium_type_name(&self) -> String {
        "boolean".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        Ok(format!("{}", this.val))
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        this.val
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(self.borrow().val == other.borrow().val)
        } else {
            None
        }
    }
}

#[derive(Hash)]
pub struct IntObject {
    pub val: BigInt,
}

impl IntObject {
    pub fn new(val: i64) -> ObjectRef {
        ObjectRef::new(IntObject {
            val: val.to_bigint().unwrap(),
        })
    }

    pub fn new_big(val: BigInt) -> ObjectRef {
        ObjectRef::new(IntObject { val })
    }

    pub fn to_i64(&self) -> RuntimeResult<i64> {
        self.val.to_i64().ok_or_else(|| {
            RuntimeError::index_too_big_error(
                "Tried to use a bigint of too large size as 64-bit integer",
            )
        })
    }
}

impl Object for ObjectCell<IntObject> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(IntObject::new_big(this.val.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "int".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        self.try_borrow().ok()?.hash(&mut hasher);
        Some(hasher.finish())
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        Ok(format!("{}", this.val))
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        this.val != 0.to_bigint().unwrap()
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(self.borrow().val == other.borrow().val)
        } else {
            None
        }
    }
}

pub struct FloatObject {
    pub val: f64,
}

impl FloatObject {
    pub fn new(val: f64) -> ObjectRef {
        ObjectRef::new(FloatObject { val })
    }
}

impl Object for ObjectCell<FloatObject> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(FloatObject::new(this.val))
    }

    fn technetium_type_name(&self) -> String {
        "int".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        let val = self.try_borrow().ok()?.val;
        hasher.write(&val.to_be_bytes());
        Some(hasher.finish())
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        let res = dtoa(this.val, DEFAULT_FLOAT_FMT);
        Ok(res)
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        this.val != 0.0
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(self.borrow().val == other.borrow().val)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub struct CharObject {
    pub val: char,
}

impl CharObject {
    pub fn new(c: char) -> ObjectRef {
        ObjectRef::new(CharObject { val: c })
    }
}

impl Object for ObjectCell<CharObject> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(this.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "char".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        self.try_borrow().ok()?.hash(&mut hasher);
        Some(hasher.finish())
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        Ok(this.val.to_string())
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        !this.val.is_whitespace()
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(self.borrow().val == other.borrow().val)
        } else {
            None
        }
    }
}

#[derive(Debug, Hash)]
pub struct StringObject {
    pub val: String,
}

impl StringObject {
    pub fn new(s: String) -> ObjectRef {
        ObjectRef::new(StringObject { val: s })
    }
}

impl Object for ObjectCell<StringObject> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let val = &this.val;
        Ok(StringObject::new(val.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "string".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        self.try_borrow().ok()?.hash(&mut hasher);
        Some(hasher.finish())
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        let val = &this.val;
        Ok(val.clone())
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        this.val != ""
    }

    fn call_method(&self, method: &str, args: &[ObjectRef], _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        tech_methods!((self, method, args) {
            "length"; () => {
                Ok(IntObject::new(this.val.len() as i64))
            },
            "escape"; () => {
                Ok(StringObject::new(this.val.escape_default().collect()))
            },
            "contains"; (arg: CharObject) => {
                Ok(BoolObject::new(this.val.contains(|c| c == arg.val)))
            },
            "lines"; () => {
                Ok(ObjectRef::new(standard::string::Lines {
                    parent: ObjectCell::clone(self),
                }))
            },
            "chars"; () => {
                Ok(ObjectRef::new(standard::string::Chars {
                    parent: ObjectCell::clone(self),
                }))
            }
        })
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(self.borrow().val == other.borrow().val)
        } else {
            None
        }
    }
}

/// A user defined function
pub struct Function {
    pub nargs: usize,
    pub name: String,
    pub context: Rc<bytecode::GlobalContext>,
    pub code: Vec<Op>,
    pub context_id: ContextId,
    pub least_ancestors: RwLock<Option<HashMap<ContextId, FrameId>>>,
}

impl Object for ObjectCell<Function> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(Function {
            nargs: this.nargs,
            name: this.name.clone(),
            context: Rc::clone(&this.context),
            code: this.code.clone(),
            context_id: this.context_id,
            least_ancestors: RwLock::new(None),
        }))
    }

    fn technetium_type_name(&self) -> String {
        "function".to_string()
    }

    fn truthy(&self) -> bool {
        true
    }

    fn call(&self, args: &[ObjectRef], context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        if args.len() != this.nargs {
            return Err(RuntimeError::type_error(format!(
                "Incorrect number of arguments given to {}: expected {}, got {}",
                this.name,
                this.nargs,
                args.len()
            )));
        }
        let mut frame = bytecode::Frame::new(
            &this.code,
            context.memory,
            Rc::clone(&this.context),
            this.least_ancestors.read().as_ref().unwrap().clone(),
            this.context_id,
        );
        for arg in args.iter().rev() {
            frame.stack.push(ObjectRef::clone(arg));
        }
        let res = frame.run();
        let fid = frame.id;
        drop(frame);
        context.memory.clear_frame(fid)?;
        res
    }
}

pub struct List {
    pub contents: Vec<ObjectRef>,
}

impl Object for ObjectCell<List> {
    fn technetium_clone(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let mut res_contents = vec![];
        for val in this.contents.iter() {
            res_contents.push(val.technetium_clone(context)?);
        }
        Ok(ObjectRef::new(List {
            contents: res_contents,
        }))
    }

    fn technetium_type_name(&self) -> String {
        "list".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        for val in self.try_borrow().ok()?.contents.iter() {
            hasher.write_u64(val.technetium_hash()?);
        }
        Some(hasher.finish())
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        let mut res = String::new();
        res.push('[');
        let mut first = true;
        for val in this.contents.iter() {
            if first {
                first = false;
            } else {
                res.push_str(", ");
            }
            res.push_str(&val.to_string()?);
        }
        res.push(']');
        Ok(res)
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        !this.contents.is_empty()
    }

    fn call_method(&self, method: &str, args: &[ObjectRef], context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        match method {
            "length" => {
                let this = self.try_borrow()?;
                if !args.is_empty() {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(this.contents.len() as i64))
                }
            }
            "contains" => {
                let this = self.try_borrow()?;
                if args.len() != 1 {
                    Err(RuntimeError::type_error("contains expects 1 arg"))
                } else {
                    Ok(BoolObject::new(this.contents.contains(&args[0])))
                }
            }
            "pop" => {
                let mut this = self.try_borrow_mut()?;
                if !args.is_empty() {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(ObjectRef::clone(&this.contents.pop().ok_or_else(|| {
                        RuntimeError::index_oob_error("Popped an empty list")
                    })?))
                }
            }
            "push" => {
                let mut this = self.try_borrow_mut()?;
                if args.len() != 1 {
                    Err(RuntimeError::type_error("push expects 1 arg"))
                } else {
                    this.contents.push(ObjectRef::clone(&args[0]));
                    Ok(UnitObject::new())
                }
            }
            "append" => {
                let mut this = self.try_borrow_mut()?;
                if args.len() != 1 {
                    Err(RuntimeError::type_error("append expects 1 arg"))
                } else {
                    let contents = &mut this.contents;
                    let iter = args[0].make_iter(context)?;

                    while let Some(val) = iter.take_iter(context)? {
                        contents.push(val);
                    }
                    Ok(UnitObject::new())
                }
            }
            _ => Err(RuntimeError::type_error(format!(
                "list has no method {}",
                method
            ))),
        }
    }

    fn make_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let iter = ListIterator {
            parent: ObjectCell::clone(self),
            index: 0,
        };

        Ok(ObjectRef::new(iter))
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let this = self.borrow();
            let other = other.borrow();
            if this.contents.len() != other.contents.len() {
                return Some(false);
            }
            for index in 0..this.contents.len() {
                if this.contents[index].technetium_eq(ObjectRef::clone(&other.contents[index]))
                    != Some(true)
                {
                    return Some(false);
                }
            }
            Some(true)
        } else {
            None
        }
    }
}

pub struct ListIterator {
    pub parent: ObjectCell<List>,
    pub index: usize,
}

impl Object for ObjectCell<ListIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(list)".to_string()
    }

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        let len = this.parent.try_borrow()?.contents.len();
        let index = &mut this.index;
        if *index >= len {
            Ok(None)
        } else {
            let old = *index;
            *index += 1;
            Ok(Some(ObjectRef::clone(
                &this.parent.try_borrow()?.contents[old],
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Slice {
    pub parent: ObjectRef,
    pub start: i64,
    pub stop: Option<i64>,
    pub step: i64,
}

impl Object for ObjectCell<Slice> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(Slice {
            parent: ObjectRef::clone(&this.parent),
            start: this.start,
            stop: this.stop,
            step: this.step,
        }))
    }

    fn technetium_type_name(&self) -> String {
        "slice".to_string()
    }

    fn make_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(SliceIterator {
            parent: ObjectRef::clone(&this.parent),
            curr: RwLock::new(this.start),
            stop: this.stop,
            step: this.step,
        }))
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        let mut res = String::new();
        if this.parent.as_any().type_id() != TypeId::of::<ObjectCell<StringObject>>() {
            res.push('[');
            let mut first = true;
            let mut curr_index = this.start;
            loop {
                if let Some(stop) = this.stop {
                    if this.step < 0 && curr_index <= stop || this.step > 0 && curr_index >= stop {
                        break;
                    }
                }
                let val = index_get(ObjectRef::clone(&this.parent), IntObject::new(curr_index));
                if let Ok(val_) = val {
                    if first {
                        first = false;
                    } else {
                        res.push_str(", ");
                    }
                    res.push_str(val_.to_string()?.as_ref());
                } else {
                    break;
                }
                curr_index += this.step;
            }
            res.push(']');
            Ok(res)
        } else {
            let mut curr_index = this.start;
            loop {
                if let Some(stop) = this.stop {
                    if this.step < 0 && curr_index <= stop || this.step > 0 && curr_index >= stop {
                        break;
                    }
                }
                let val = index_get(ObjectRef::clone(&this.parent), IntObject::new(curr_index));
                if let Ok(val_) = val {
                    res.push_str(val_.to_string()?.as_ref());
                } else {
                    break;
                }
                curr_index += this.step;
            }
            Ok(res)
        }
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        // This error handling is weird... It means that if this slice is currently in use, eq will probably always return false.
        let this = self.try_borrow().ok()?;
        // Check indices in order in the other container, to see if they're equal to our values
        let mut curr_index = this.start;
        let mut curr_index_0 = 0i64;
        loop {
            if let Some(stop) = this.stop {
                if this.step < 0 && curr_index <= stop || this.step > 0 && curr_index >= stop {
                    let other_val = index_get(ObjectRef::clone(&other), IntObject::new(curr_index_0));
                    if !other_val.is_err() {
                        // other iterator has more entries
                        return Some(false);
                    }
                    break;
                }
            }
            let my_val = index_get(ObjectRef::clone(&this.parent), IntObject::new(curr_index));
            let other_val = index_get(ObjectRef::clone(&other), IntObject::new(curr_index_0));
            if my_val.is_err() && other_val.is_err() {
                return Some(true);
            }
            if my_val.is_err() || other_val.is_err() {
                debug!("A");
                return Some(false);
            }
            if my_val.unwrap() != other_val.unwrap() {
                debug!("B");
                return Some(false);
            }
            curr_index += this.step;
            curr_index_0 += 1;
        }
        Some(true)
    }
}

#[derive(Debug)]
pub struct SliceIterator {
    pub parent: ObjectRef,
    pub curr: RwLock<i64>,
    pub stop: Option<i64>,
    pub step: i64,
}

impl Object for ObjectCell<SliceIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(slice)".to_string()
    }

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let this = self.try_borrow()?;
        let mut curr_ = this.curr.write();
        if let Some(stop) = this.stop {
            if this.step < 0 && *curr_ <= stop || this.step > 0 && *curr_ >= stop {
                return Ok(None);
            }
        }
        let old = index_get(ObjectRef::clone(&this.parent), IntObject::new(*curr_));
        *curr_ += this.step;
        Ok(old.ok())
    }
}

pub struct Tuple {
    pub contents: Vec<ObjectRef>,
}

impl Object for ObjectCell<Tuple> {
    fn technetium_clone(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let mut res_contents = vec![];
        for val in this.contents.iter() {
            res_contents.push(val.technetium_clone(context)?);
        }
        Ok(ObjectRef::new(Tuple {
            contents: res_contents,
        }))
    }
    
    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        let mut res = String::new();
        res.push('(');
        let mut first = true;
        for item in this.contents.iter() {
            if first {
                first = false;
            } else {
                res.push_str(", ");
            }
            res.push_str(item.to_string()?.as_ref());
        }
        if this.contents.len() == 1 {
            res.push(',');
        }
        res.push(')');
        Ok(res)
    }

    fn technetium_type_name(&self) -> String {
        "tuple".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        for val in self.try_borrow().ok()?.contents.iter() {
            hasher.write_u64(val.technetium_hash()?);
        }
        Some(hasher.finish())
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        !this.contents.is_empty()
    }

    fn call_method(&self, method: &str, args: &[ObjectRef], _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        tech_methods!((self, method, args) {
            "length"; () => {
                Ok(IntObject::new(this.contents.len() as i64))
            },
            "contains"; (; arg) => {
                Ok(BoolObject::new(this.contents.contains(arg)))
            }
        })
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let this = self.borrow();
            let other = other.borrow();
            if this.contents.len() != other.contents.len() {
                return Some(false);
            }
            for index in 0..this.contents.len() {
                if this.contents[index].technetium_eq(ObjectRef::clone(&other.contents[index]))
                    != Some(true)
                {
                    return Some(false);
                }
            }
            Some(true)
        } else {
            None
        }
    }
}

pub struct UnitObject;

impl UnitObject {
    pub fn new() -> ObjectRef {
        ObjectRef::new(UnitObject)
    }
}

impl Object for ObjectCell<UnitObject> {
    fn technetium_type_name(&self) -> String {
        "unit".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        Ok("unit".to_string())
    }

    fn truthy(&self) -> bool {
        false
    }
    
    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if other.as_any().downcast_ref::<Self>().is_some() {
            Some(true)
        } else {
            None
        }
    }
}

pub struct Set {
    pub contents: HashSet<HashableObjectRef>,
}

impl Object for ObjectCell<Set> {
    fn technetium_clone(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let mut res_contents = HashSet::new();
        for val in this.contents.iter() {
            res_contents.insert(val.technetium_clone(context)?.hashable().unwrap());
        }
        Ok(ObjectRef::new(Set {
            contents: res_contents,
        }))
    }

    fn technetium_type_name(&self) -> String {
        "set".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        for val in self.try_borrow().ok()?.contents.iter() {
            hasher.write_u64(val.technetium_hash()?);
        }
        Some(hasher.finish())
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        let mut res = String::new();
        res.push('{');
        let mut first = true;
        for val in this.contents.iter() {
            if first {
                first = false;
            } else {
                res.push_str(", ");
            }
            res.push_str(&val.to_string()?);
        }
        res.push('}');
        Ok(res)
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        !this.contents.is_empty()
    }
    
    fn make_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let setiterbuild = SetIteratorBuilder {
            parent: ObjectCell::clone(self),
            s_builder: |head| head.try_borrow().unwrap(),
            siter_builder: |s| s.contents.iter(),
        };

        Ok(ObjectRef::new(setiterbuild.build()))
    }

    fn call_method(&self, method: &str, args: &[ObjectRef], _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        tech_methods!((self, method, args) {
            "length"; () => {
                let this = self.try_borrow()?;
                Ok(IntObject::new(this.contents.len() as i64))
            },
            "contains"; (; arg) => {
                let this = self.try_borrow()?;
                let hashable = arg.hashable().ok_or_else(|| {
                    RuntimeError::type_error("value must be hashable to check for containment")
                })?;
                Ok(BoolObject::new(this.contents.contains(&hashable)))
            },
            "add"; (; arg) => {
                let mut this = self.try_borrow_mut()?;
                this.contents.insert(arg.hashable().ok_or_else(|| {
                    RuntimeError::type_error("value must be hashable to be added to a set")
                })?);
                arg.lock_immutable();
                Ok(UnitObject::new())
            },
            "remove"; (; arg) => {
                let mut this = self.try_borrow_mut()?;
                let res = this.contents.remove(&arg.hashable().ok_or_else(|| {
                    RuntimeError::type_error("value must be hashable to be added to a set")
                })?);
                Ok(BoolObject::new(res))
            }
        })
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let this = self.borrow();
            let other = other.borrow();
            Some(this.contents == other.contents)
        } else {
            None
        }
    }
}

#[self_referencing]
pub struct SetIterator {
    parent: ObjectCell<Set>,
    #[covariant]
    #[borrows(parent)]
    s: std::cell::Ref< 'this,Set>,
    #[not_covariant]
    #[borrows(s)]
    siter: std::collections::hash_set::Iter<'this, HashableObjectRef>
}

impl Object for ObjectCell<SetIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(slice)".to_string()
    }

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        Ok(this.with_siter_mut(|siter| {
            siter.next().map(|val| val.clone().downgrade())
        }))
    }
}

pub struct Dictionary {
    pub contents: HashMap<HashableObjectRef, ObjectRef>,
}

impl Object for ObjectCell<Dictionary> {
    fn technetium_clone(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let mut res_contents = HashMap::new();
        for (key, val) in this.contents.iter() {
            res_contents.insert(
                key.technetium_clone(context)?.hashable().unwrap(),
                val.technetium_clone(context)?,
            );
        }
        Ok(ObjectRef::new(Dictionary {
            contents: res_contents,
        }))
    }

    fn technetium_type_name(&self) -> String {
        "dictionary".to_string()
    }

    fn technetium_hash(&self) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        for (key, val) in self.try_borrow().ok()?.contents.iter() {
            hasher.write_u64(key.technetium_hash()?);
            hasher.write_u64(val.technetium_hash()?);
        }
        Some(hasher.finish())
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let this = self.try_borrow()?;
        let mut res = String::new();
        res.push('{');
        let mut first = true;
        for (key, val) in this.contents.iter() {
            if first {
                first = false;
            } else {
                res.push_str(", ");
            }
            res.push_str(&key.to_string()?);
            res.push_str(": ");
            res.push_str(&val.to_string()?);
        }
        res.push('}');
        Ok(res)
    }

    fn truthy(&self) -> bool {
        let this = self.borrow();
        !this.contents.is_empty()
    }

    fn call_method(&self, method: &str, args: &[ObjectRef], _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        tech_methods!((self, method, args) {
            "length"; () => {
                Ok(IntObject::new(this.contents.len() as i64))
            }
        })
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let this = self.borrow();
            let other = other.borrow();
            Some(this.contents == other.contents)
        } else {
            None
        }
    }
}
