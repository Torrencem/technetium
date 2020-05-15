
pub mod bytecode;
pub mod builtins;
pub mod standard;
pub mod memory;
pub mod error;

#[macro_use]
extern crate rental;

#[macro_use]
extern crate lazy_static;

use bytecode::Op;
use bytecode::{ContextId, FrameId};
use builtins::index_get;
use memory::*;
use error::*;
use std::any::Any;
use std::any::TypeId;
use std::clone::Clone as RustClone;
use std::collections::HashMap;
use std::rc::Rc;
use mlrefcell::MLRefCell;
use parking_lot::RwLock;
use std::ops::{Deref, DerefMut};
use num::bigint::{BigInt, ToBigInt};
use num::cast::ToPrimitive;
use stable_deref_trait::StableDeref;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::HashSet;

use dtoa;

use std::fmt;

#[repr(transparent)]
#[derive(Debug)]
pub struct ObjectRef {
    inner: Box<dyn Object>,
}

impl Deref for ObjectRef {
    type Target = Box<dyn Object>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ObjectRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

unsafe impl StableDeref for ObjectRef { }

/// An object reference that's guaranteed to have a valid hash (that doesn't throw errors)
///
/// Construct a `HashableObjectRef` using the `hashable` method of `ObjectRef`
///
/// Unlike ObjectRef, HashableObjectRef implements Hash
#[derive(Debug)]
pub struct HashableObjectRef {
    inner: Box<dyn Object>,
}

impl Deref for HashableObjectRef {
    type Target = Box<dyn Object>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for HashableObjectRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Hash for HashableObjectRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.technetium_hash().unwrap());
    }
}

impl PartialEq for HashableObjectRef {
    fn eq(&self, other: &HashableObjectRef  ) -> bool {
        match self.technetium_eq(other.opaque_clone()) {
            Some(val) => val,
            None => {
                self.ref_eq(other.opaque_clone())
            }
        }
    }
}

impl Eq for HashableObjectRef { }

impl Clone for HashableObjectRef {
    fn clone(&self) -> Self {
        self.inner.opaque_clone().hashable().unwrap()
    }
}

impl ObjectRef {
    pub fn new_from_cell<T>(obj: ObjectCell<T>) -> Self
    where ObjectCell<T>: Object {
        ObjectRef {
            inner: Box::new(obj),
        }
    }

    pub fn new<T>(inner: T) -> Self
    where ObjectCell<T>: Object {
        ObjectRef {
            inner: Box::new(ObjectCell::new(inner)),
        }
    }
    
    /// Create a HashableObjectRef, by checking if it has a valid hash
    pub fn hashable(&self) -> Option<HashableObjectRef> {
        if self.technetium_hash().is_none() {
            None
        } else {
            let new = self.opaque_clone();
            Some(HashableObjectRef { inner: new.inner })
        }
    }
}

impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        self.inner.opaque_clone()
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct ObjectCell<T>
where ObjectCell<T>: Object {
    inner: Rc<MLRefCell<T>>,
}

impl<T> Clone for ObjectCell<T>
where ObjectCell<T>: Object {
    fn clone(&self) -> Self {
        ObjectCell {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> ObjectCell<T> 
where ObjectCell<T>: Object {
    pub fn new(val: T) -> Self {
        ObjectCell {
            inner: Rc::new(MLRefCell::new(val)),
        }
    }
    /// Lock the `MLRefCell` inside the cell, effectively making the value immutable, giving a
    /// runtime error for any future mutation of the object. This is useful if the value will need
    /// to be used as the key in a HashMap, or in a HashSet
    pub fn lock(&self) {
        let _ = self.inner.lock();
    }
}

impl<T> Deref for ObjectCell<T>
where ObjectCell<T>: Object {
    type Target = Rc<MLRefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for ObjectCell<T>
where ObjectCell<T>: Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

unsafe impl<T> StableDeref for ObjectCell<T>
where ObjectCell<T>: Object { }

pub trait ToAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Object> ToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait OpaqueClone {
    fn opaque_clone(&self) -> ObjectRef;
}

impl<T> OpaqueClone for ObjectCell<T>
where ObjectCell<T>: Object {
    fn opaque_clone(&self) -> ObjectRef {
        let self_copy = ObjectCell {
            inner: Rc::clone(&self.inner),
        };
        ObjectRef {
            inner: Box::new(self_copy),
        }
    }
}

pub trait RawPointer {
    fn raw_pointer(&self) -> *const ();
}

impl<T> RawPointer for ObjectCell<T>
where ObjectCell<T>: Object {
    fn raw_pointer(&self) -> *const () {
        self.as_ptr() as *const ()
    }
}

pub trait LockImmutable {
    fn lock_immutable(&self);
}

impl<T> LockImmutable for ObjectCell<T>
where ObjectCell<T>: Object {
    fn lock_immutable(&self) {
        self.lock();
    }
}

pub trait Object: Any + ToAny + OpaqueClone + RawPointer + LockImmutable {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!(
            "{} can not be cloned",
            self.technetium_type_name()
        )))
    }

    fn technetium_hash(&self) -> Option<u64> {
        None
    }

    fn technetium_type_name(&self) -> String;

    fn to_string(&self) -> RuntimeResult<String> {
        Ok(format!(
            "<{}>",
            self.technetium_type_name()
        ))
    }

    fn get_attr(&self, _attr: String) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!(
            "{} has no attributes",
            self.technetium_type_name()
        )))
    }

    fn set_attr(&self, _attr: String, _val: ObjectRef) -> RuntimeResult<()> {
        Err(RuntimeError::attribute_error(format!(
            "Cannot set attributes of {}",
            self.technetium_type_name()
        )))
    }

    fn call_method(&self, _method: &str, _args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!(
            "Cannot call method of {}",
            self.technetium_type_name()
        )))
    }

    fn call(
        &self,
        _args: &[ObjectRef],
        _locals: &mut MemoryManager,
    ) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!(
            "Object of type {} is not callable",
            self.technetium_type_name()
        )))
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!(
            "Object of type {} cannot be made into an iterator",
            self.technetium_type_name()
        )))
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        Err(RuntimeError::type_error(format!(
            "Object of type {} cannot be iterated",
            self.technetium_type_name()
        )))
    }

    fn truthy(&self) -> bool {
        true
    }

    fn technetium_eq(&self, _other: ObjectRef) -> Option<bool> {
        None
    }
    
    fn ref_eq(&self, other: ObjectRef) -> bool {
        self.raw_pointer() == other.raw_pointer()
    }
}

impl fmt::Debug for dyn Object {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.to_string().unwrap_or("Object".to_string()))
    }
}

impl PartialEq for ObjectRef {
    fn eq(&self, other: &ObjectRef) -> bool {
        match self.technetium_eq(ObjectRef::clone(other)) {
            Some(val) => val,
            None => {
                self.ref_eq(ObjectRef::clone(other))
            }
        }
    }
}

impl Eq for ObjectRef { }

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
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
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
        let res = ObjectRef::new(IntObject { val: val.to_bigint().unwrap() });
        res
    }

    pub fn new_big(val: BigInt) -> ObjectRef {
        let res = ObjectRef::new(IntObject { val });
        res
    }

    pub fn to_i64(&self) -> RuntimeResult<i64> {
        self.val.to_i64().ok_or_else(|| RuntimeError::index_too_big_error("Tried to use a bigint of too large size as 64-bit integer"))
    }
}

impl Object for ObjectCell<IntObject> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
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
        let res = ObjectRef::new(FloatObject { val });
        res
    }
}

impl Object for ObjectCell<FloatObject> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
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
        let mut res: Vec<u8> = vec![];
        dtoa::write(&mut res, this.val)?;
        Ok(String::from_utf8(res).unwrap())
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
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
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
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
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

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        match method {
            "length" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(this.val.len() as i64))
                }
            },
            "escape" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(StringObject::new(this.val.escape_default().collect()))
                }
            },
            "lines" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("lines expects 0 args"))
                } else {
                    Ok(ObjectRef::new(standard::string::Lines {
                        parent: ObjectCell::clone(self),
                    }))
                }
            },
            "chars" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("chars expects 0 args"))
                } else {
                    Ok(ObjectRef::new(standard::string::Chars {
                        parent: ObjectCell::clone(self),
                    }))
                }
            }
            _ => Err(RuntimeError::type_error(format!(
                "string has no method {}",
                method
            ))),
        }
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Some(self.borrow().val == other.borrow().val)
        } else {
            None
        }
    }
}

pub struct Function {
    pub nargs: usize,
    pub name: String,
    pub context: Rc<bytecode::GlobalContext>,
    pub code: Vec<Op>,
    pub context_id: ContextId,
    pub least_ancestors: RwLock<Option<HashMap<ContextId, FrameId>>>,
}

impl Object for ObjectCell<Function> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
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

    fn call(
        &self,
        args: &[ObjectRef],
        locals: &mut MemoryManager,
    ) -> RuntimeResult<ObjectRef> {
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
            locals,
            Rc::clone(&this.context),
            this.least_ancestors
                .read()
                .as_ref()
                .unwrap()
                .clone(),
            this.context_id,
        );
        for arg in args.iter().rev() {
            frame.stack.push(ObjectRef::clone(arg));
        }
        let res = frame.run();
        let fid = frame.id;
        drop(frame);
        locals.clear_frame(fid)?;
        res
    }
}

pub struct List {
    pub contents: Vec<ObjectRef>,
}

impl Object for ObjectCell<List> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let mut res_contents = vec![];
        for val in this.contents.iter() {
            res_contents.push(val.technetium_clone()?);
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
        this.contents.len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        match method {
            "length" => {
                let this = self.try_borrow()?;
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(this.contents.len() as i64))
                }
            }
            "pop" => {
                let mut this = self.try_borrow_mut()?;
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(ObjectRef::clone(&this.contents.pop().ok_or(RuntimeError::index_oob_error("Popped an empty list"))?))
                }
            }
            "push" => {
                let mut this = self.try_borrow_mut()?;
                if args.len() != 1 {
                    Err(RuntimeError::type_error("push expects 1 arg"))
                } else {
                    this.contents.push(ObjectRef::clone(&args[0]));
                    Ok(VoidObject::new())
                }
            }
            "append" => {
                let mut this = self.try_borrow_mut()?;
                if args.len() != 1 {
                    Err(RuntimeError::type_error("append expects 1 arg"))
                } else {
                    let contents = &mut this.contents;
                    let iter = args[0].make_iter()?;

                    while let Some(val) = iter.take_iter()? {
                        contents.push(val);
                    }
                    Ok(VoidObject::new())
                }
            }
            _ => Err(RuntimeError::type_error(format!(
                "list has no method {}",
                method
            ))),
        }
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
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
                if this.contents[index].technetium_eq(ObjectRef::clone(&other.contents[index])) != Some(true) {
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

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        let len = this.parent.try_borrow()?.contents.len();
        let index = &mut this.index;
        if *index >= len {
            Ok(None)
        } else {
            let old = *index;
            *index += 1;
            Ok(Some(ObjectRef::clone(&this.parent.try_borrow()?.contents[old])))
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
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
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

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
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
                    if this.step < 0 && curr_index <= stop
                    || this.step > 0 && curr_index >= stop {
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
                    if this.step < 0 && curr_index <= stop
                    || this.step > 0 && curr_index >= stop {
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

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let this = self.try_borrow()?;
        let mut curr_ = this.curr.write();
        if let Some(stop) = this.stop {
            if this.step < 0 && *curr_ <= stop
            || this.step > 0 && *curr_ >= stop {
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
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let mut res_contents = vec![];
        for val in this.contents.iter() {
            res_contents.push(val.technetium_clone()?);
        }
        Ok(ObjectRef::new(Tuple {
            contents: res_contents,
        }))
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
        this.contents.len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        match method {
            "length" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(this.contents.len() as i64))
                }
            }
            _ => Err(RuntimeError::type_error(format!(
                "list has no method {}",
                method
            ))),
        }
    }

    fn technetium_eq(&self, other: ObjectRef) -> Option<bool> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let this = self.borrow();
            let other = other.borrow();
            if this.contents.len() != other.contents.len() {
                return Some(false);
            }
            for index in 0..this.contents.len() {
                if this.contents[index].technetium_eq(ObjectRef::clone(&other.contents[index])) != Some(true) {
                    return Some(false);
                }
            }
            Some(true)
        } else {
            None
        }
    }
}

pub struct VoidObject;

impl VoidObject {
    pub fn new() -> ObjectRef {
        ObjectRef::new(VoidObject)
    }
}

impl Object for ObjectCell<VoidObject> {
    fn technetium_type_name(&self) -> String {
        "void".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        Ok("void".to_string())
    }

    fn truthy(&self) -> bool {
        false
    }
}

pub struct Set {
    pub contents: HashSet<HashableObjectRef>,
}

impl Object for ObjectCell<Set> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let mut res_contents = vec![];
        for val in this.contents.iter() {
            res_contents.push(val.technetium_clone()?);
        }
        Ok(ObjectRef::new(List {
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
        this.contents.len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        match method {
            "length" => {
                let this = self.try_borrow()?;
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(this.contents.len() as i64))
                }
            }
            "add" => {
                let mut this = self.try_borrow_mut()?;
                if args.len() != 1 {
                    Err(RuntimeError::type_error("add expects 1 arg"))
                } else {
                    this.contents.insert(args[0].hashable().ok_or(RuntimeError::type_error("value must be hashable to be added to a set"))?);
                    args[0].lock_immutable();
                    Ok(VoidObject::new())
                }
            }
            "remove" => {
                let mut this = self.try_borrow_mut()?;
                if args.len() != 1 {
                    Err(RuntimeError::type_error("add expects 1 arg"))
                } else {
                    let res = this.contents.remove(&args[0].hashable().ok_or(RuntimeError::type_error("value must be hashable to be added to a set"))?);
                    Ok(BoolObject::new(res))
                }
            }
            _ => Err(RuntimeError::type_error(format!(
                "set has no method {}",
                method
            ))),
        }
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
