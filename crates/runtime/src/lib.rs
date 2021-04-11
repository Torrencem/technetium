pub mod builtins;
pub mod bytecode;
pub mod error;
pub mod memory;
pub mod standard;
pub mod core_objects;
pub mod prelude;

#[macro_use]
extern crate rental;

#[macro_use]
extern crate lazy_static;

use error::*;
use memory::*;
use mlrefcell::MLRefCell;
use once_cell::sync::OnceCell;
use stable_deref_trait::StableDeref;
use pretty_dtoa::FmtFloatConfig;
use std::any::Any;
use std::clone::Clone as RustClone;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::path::PathBuf;
use std::fs;

use std::fmt;

/// Conditionally execute code depending on if an ObjectRef contains an inner object of a certain
/// type or not. This helps greatly improve readability in internal code
#[macro_export]
macro_rules! downcast {
    // TODO: add an optional 'mut' before the identifier, which will try_borrow_mut instead of
    // try_borrow
    (($id:ident : $type:ty = $val:expr) -> $main:block else $other:block) => {
        if let Some($id) = $val.as_any().downcast_ref::<ObjectCell<$type>>() {
            let $id = $id.try_borrow()?;
            $main
        } else {
            $other
        }
    };
    (($id:ident : $type:ty = $val:expr) -> $main:block) => {
        if let Some($id) = $val.as_any().downcast_ref::<ObjectCell<$type>>() {
            let $id = $id.try_borrow()?;
            $main
        }
    };
}

/// Dynamically dispatch to different code depending on the types of two variables. Useful for
/// defining readable and short internal code for operations
#[macro_export]
macro_rules! match_tech_types {
    (($a:expr, $b:expr) { $(($v1:ident : $t1:ty, $v2:ident : $t2:ty)=>$b1:block),* , _ => $b2:block } ) => {{
        let a_any__ = $a.as_any();
        let b_any__ = $b.as_any();
        match (a_any__.type_id(), b_any__.type_id()) {
            $( 
                (a__, b__)
                    if a__ == TypeId::of::<ObjectCell<$t1>>()
                    && b__ == TypeId::of::<ObjectCell<$t2>>() =>
                    {
                        let $v1 = a_any__
                            .downcast_ref::<ObjectCell<$t1>>()
                            .unwrap()
                            .try_borrow()?;
                        let $v2 = b_any__
                            .downcast_ref::<ObjectCell<$t2>>()
                            .unwrap()
                            .try_borrow()?;
                        $b1
                    }
            ),*
            _ => {
                $b2
            }
        }
    }};
}

pub static DEFAULT_FLOAT_FMT: FmtFloatConfig = FmtFloatConfig::default();

pub static PARSED_CLARGS: OnceCell<Vec<String>> = OnceCell::new();

/// The parent directory of the script that tc is being run in, or the parent directory from where
/// the user called the ``tech`` binary. Useful for locating the .tcmake folder
pub static INVOKE_ABSOLUTE_PARENT_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn get_tcmake_dir() -> Option<PathBuf> {
    INVOKE_ABSOLUTE_PARENT_DIR.get().cloned().map(|mut path| {
        path.push(".tcmake");
        if !path.exists() {
            let _ = fs::create_dir(&path);
        }
        path
    })
}

/// The main object reference type, which can be passed around to represent
/// an object of any valid Object type
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

unsafe impl StableDeref for ObjectRef {}

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
    fn eq(&self, other: &HashableObjectRef) -> bool {
        match self.technetium_eq(other.opaque_clone()) {
            Some(val) => val,
            None => self.ref_eq(other.opaque_clone()),
        }
    }
}

impl Eq for HashableObjectRef {}

impl Clone for HashableObjectRef {
    fn clone(&self) -> Self {
        self.inner.opaque_clone().hashable().unwrap()
    }
}

impl ObjectRef {
    pub fn new_from_cell<T>(obj: ObjectCell<T>) -> Self
    where
        ObjectCell<T>: Object,
    {
        ObjectRef {
            inner: Box::new(obj),
        }
    }

    pub fn new<T>(inner: T) -> Self
    where
        ObjectCell<T>: Object,
    {
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

/// The universal container object for implementers of the Object trait.
/// Anything that implements Object should be of the form ObjectCell<T>.
/// ObjectCell combines interior mutability and shared ownership, so it
/// is the primary container used for objects in technetium
#[repr(transparent)]
#[derive(Debug)]
pub struct ObjectCell<T>
where
    ObjectCell<T>: Object,
{
    inner: Rc<MLRefCell<T>>,
}

impl<T> Clone for ObjectCell<T>
where
    ObjectCell<T>: Object,
{
    fn clone(&self) -> Self {
        ObjectCell {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> ObjectCell<T>
where
    ObjectCell<T>: Object,
{
    pub fn new(val: T) -> Self {
        ObjectCell {
            inner: Rc::new(MLRefCell::new(val)),
        }
    }
    /// Lock the ``MLRefCell`` inside the cell, effectively making the value immutable, giving a
    /// runtime error for any future mutation of the object. This is useful if the value will need
    /// to be used as the key in a HashMap, or in a HashSet
    pub fn lock(&self) {
        let _ = self.inner.lock();
    }
}

impl<T> Deref for ObjectCell<T>
where
    ObjectCell<T>: Object,
{
    type Target = Rc<MLRefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for ObjectCell<T>
where
    ObjectCell<T>: Object,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

unsafe impl<T> StableDeref for ObjectCell<T> where ObjectCell<T>: Object {}

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
where
    ObjectCell<T>: Object,
{
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
where
    ObjectCell<T>: Object,
{
    fn raw_pointer(&self) -> *const () {
        self.as_ptr() as *const ()
    }
}

pub trait LockImmutable {
    fn lock_immutable(&self);
}

impl<T> LockImmutable for ObjectCell<T>
where
    ObjectCell<T>: Object,
{
    fn lock_immutable(&self) {
        self.lock();
    }
}

pub struct RuntimeContext<'a> {
    pub memory: &'a mut MemoryManager,
}

/// The primary trait for objects in technetium.
///
/// Types that implement ``Object``
/// should be of the form ``ObjectCell<T>`` for some T. This will give all of the
/// requirement subtraits for free.
pub trait Object: Any + ToAny + OpaqueClone + RawPointer + LockImmutable {
    /// Create a deep clone of an object. This is primarily used in the ``clone``
    /// function in technetium
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!(
            "{} can not be cloned",
            self.technetium_type_name()
        )))
    }
    
    /// Hash an object. This is not required, so the default implementation always
    /// returns ``None``. 
    ///
    /// In implementing a hash, it's important that ``x == y``
    /// implies that ``x.technetium_hash() == y.technetium_hash()`` to avoid logic
    /// errors.
    ///
    /// Note that ObjectRef does not implement ``Hash`` in Rust, but HashableObjectRef
    /// does. See the docs for [HashableObjectRef](struct.HashableObjectRef.html) for
    /// more information
    fn technetium_hash(&self) -> Option<u64> {
        None
    }
    
    /// A type name for an object.
    ///
    /// Conventions are that type names are all lowercase, and use parentheses to denote
    /// "sub-types" (for example: "iterator(list)")
    ///
    /// This function should not fail, so should return a set value.
    fn technetium_type_name(&self) -> String;
    
    /// Convert an object to a String.
    fn to_string(&self) -> RuntimeResult<String> {
        // TODO fix: This should be passed some way to ask "Am I the recursive call of myself" (i.e.
        // should I just return [...] for lists). This might be able to be attached to a
        // RuntimeContext<'_>
        Ok(format!("<{}>", self.technetium_type_name()))
    }
    
    /// Get an attribute of an object
    fn get_attr(&self, _attr: String, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!(
            "{} has no attributes",
            self.technetium_type_name()
        )))
    }
    
    /// Set an attribute of an object
    fn set_attr(&self, _attr: String, _val: ObjectRef, _context: &mut RuntimeContext<'_>) -> RuntimeResult<()> {
        Err(RuntimeError::attribute_error(format!(
            "Cannot set attributes of {}",
            self.technetium_type_name()
        )))
    }
    
    /// Call a given method of an object
    fn call_method(&self, _method: &str, _args: &[ObjectRef], _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!(
            "Cannot call method of {}",
            self.technetium_type_name()
        )))
    }
    
    /// Call a given object as a function.
    ///
    /// This takes a memory manager
    /// primarily for the [Function](struct.Function.html) object,
    /// which needs to be able to reference and change locals.
    fn call(&self, _args: &[ObjectRef], _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!(
            "Object of type {} is not callable",
            self.technetium_type_name()
        )))
    }
    
    /// Create an iterator over an object. This is used for initializing
    /// ``for`` loops.
    fn make_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!(
            "Object of type {} cannot be made into an iterator",
            self.technetium_type_name()
        )))
    }
    
    /// Take from this object, assuming it is an iterator. This is used for
    /// stepping through ``for`` loops.
    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        Err(RuntimeError::type_error(format!(
            "Object of type {} cannot be iterated",
            self.technetium_type_name()
        )))
    }
    
    /// Determine whether an object is "truthy" (whether it should be treated
    /// as true when used as a boolean)
    fn truthy(&self) -> bool {
        true
    }
    
    /// Equal-as-value (like == in Python, or .equals() in Java)
    fn technetium_eq(&self, _other: ObjectRef) -> Option<bool> {
        None
    }
    
    /// Equal-as-reference (like ``is`` in Python, or == in Java)
    ///
    /// This is treated as a fallback in the ``Eq`` implementation
    /// for ``ObjectRef``, primarily for sets and dictionaries
    fn ref_eq(&self, other: ObjectRef) -> bool {
        self.raw_pointer() == other.raw_pointer()
    }
}

impl fmt::Debug for dyn Object {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            self.to_string().unwrap_or_else(|_| "Object".to_string())
        )
    }
}

impl PartialEq for ObjectRef {
    fn eq(&self, other: &ObjectRef) -> bool {
        match self.technetium_eq(ObjectRef::clone(other)) {
            Some(val) => val,
            None => self.ref_eq(ObjectRef::clone(other)),
        }
    }
}

impl Eq for ObjectRef {}

