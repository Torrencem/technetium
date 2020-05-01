
use std::sync::Arc;
use std::any::Any;
use std::clone::Clone as RustClone;
use crate::bytecode::Op;
use crate::bytecode;
use crate::bytecode::{NonLocalName, ContextId, FrameId};
use std::sync::Mutex;
use std::collections::HashMap;

use dtoa;

use std::fmt;

use crate::error::*;

pub type ObjectRef = Arc<dyn Object>;

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

pub trait Object : Any + ToAny + Send + Sync {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!("{} can not be cloned", self.technetium_type_name())))
    }

    fn technetium_type_name(&self) -> String;

    fn to_string(&self) -> RuntimeResult<String> {
        Err(RuntimeError::type_error(format!("{} can not be converted into a string", self.technetium_type_name())))
    }

    fn get_attr(&self, attr: String) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!("{} has no attributes", self.technetium_type_name())))
    }

    fn set_attr(&self, attr: String, val: ObjectRef) -> RuntimeResult<()> {
        Err(RuntimeError::attribute_error(format!("Cannot set attributes of {}", self.technetium_type_name())))
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!("Cannot call method of {}", self.technetium_type_name())))
    }

    fn call(&self, args: &[ObjectRef], locals: &mut HashMap<NonLocalName, ObjectRef>) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!("Object of type {} is not callable", self.technetium_type_name())))
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!("Object of type {} cannot be made into an iterator", self.technetium_type_name())))
    }
    
    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        Err(RuntimeError::type_error(format!("Object of type {} cannot be iterated", self.technetium_type_name())))
    }

    fn truthy(&self) -> bool {
        true
    }
}

impl fmt::Debug for dyn Object {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.to_string().unwrap_or("Object".to_string()))
    }
}

pub struct BoolObject {
    pub val: bool,
}

impl BoolObject {
    pub fn new(val: bool) -> ObjectRef {
        Arc::new(BoolObject { val })
    }
}

impl Object for BoolObject {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(BoolObject::new(self.val))
    }

    fn technetium_type_name(&self) -> String {
        "boolean".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        Ok(format!("{}", self.val))
    }

    fn truthy(&self) -> bool {
        self.val
    }
}

pub struct IntObject {
    pub val: i64,
}

impl IntObject {
    pub fn new(val: i64) -> ObjectRef {
        let res = Arc::new(IntObject { val });
        res
    }
}

impl Object for IntObject {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(IntObject::new(self.val))
    }

    fn technetium_type_name(&self) -> String {
        "int".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        Ok(format!("{}", self.val))
    }

    fn truthy(&self) -> bool {
        self.val != 0
    }
}

pub struct FloatObject {
    pub val: f64,
}

impl FloatObject {
    pub fn new(val: f64) -> ObjectRef {
        let res = Arc::new(FloatObject { val });
        res
    }
}

impl Object for FloatObject {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(FloatObject::new(self.val))
    }

    fn technetium_type_name(&self) -> String {
        "int".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let mut res: Vec<u8> = vec![];
        dtoa::write(&mut res, self.val);
        Ok(String::from_utf8(res).unwrap())
    }

    fn truthy(&self) -> bool {
        self.val != 0.0
    }
}

#[derive(Clone, Debug)]
pub struct StringObject {
    pub val: String
}

impl StringObject {
    pub fn new(s: String) -> ObjectRef {
        Arc::new(StringObject { val: s })
    }
}

impl Object for StringObject {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(RustClone::clone(self)))
    }

    fn technetium_type_name(&self) -> String {
        "string".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        Ok(RustClone::clone(&self.val))
    }

    fn truthy(&self) -> bool {
        self.val != ""
    }
}

pub struct Function {
    pub nargs: usize,
    pub name: String,
    pub context: Arc<bytecode::GlobalContext>,
    pub code: Vec<Op>,
    pub context_id: ContextId,
    pub least_ancestors: Mutex<Option<HashMap<ContextId, FrameId>>>,
}

impl Object for Function {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(Function {
            nargs: self.nargs,
            name: self.name.clone(),
            context: Arc::clone(&self.context),
            code: self.code.clone(),
            context_id: self.context_id,
            least_ancestors: Mutex::new(None),
        }))
    }

    fn technetium_type_name(&self) -> String {
        "function".to_string()
    }

    fn truthy(&self) -> bool {
        true
    }
    
    fn call(&self, args: &[ObjectRef], locals: &mut HashMap<NonLocalName, ObjectRef>) -> RuntimeResult<ObjectRef> {
        if args.len() != self.nargs {
            return Err(RuntimeError::type_error(format!("Incorrect number of arguments given to {}: expected {}, got {}", self.name, self.nargs, args.len())));
        }
        let mut frame = bytecode::Frame::new(&self.code, locals, Arc::clone(&self.context), self.least_ancestors.lock().unwrap().as_ref().unwrap().clone(), self.context_id);
        for arg in args.iter().rev() {
            frame.stack.push(Arc::clone(arg));
        }
        frame.run()
    }
}


pub struct List {
    pub contents: Mutex<Vec<ObjectRef>>,
}

impl Object for List {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let mut res_contents = vec![];
        let contents_ = self.contents.lock().unwrap();
        for val in contents_.iter() {
            res_contents.push(val.technetium_clone()?);
        }
        Ok(Arc::new(List { contents: Mutex::new(res_contents) }))
    }

    fn technetium_type_name(&self) -> String {
        "list".to_string()
    }

    fn truthy(&self) -> bool {
        self.contents.lock().unwrap().len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
       match method {
            "length" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args".to_string()))
                } else {
                    Ok(IntObject::new(self.contents.lock().unwrap().len() as i64))
                }
            },
            _ => {
                Err(RuntimeError::type_error(format!("list has no method {}", method)))
            },
       }
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        let iter = ListIterator {
            contents: self.contents.lock().unwrap().iter().map(|val| Arc::clone(val)).collect(),
            index: Mutex::new(0),
        };

        Ok(Arc::new(iter))
    }
}

pub struct ListIterator {
    pub contents: Vec<ObjectRef>,
    pub index: Mutex<usize>,
}

impl Object for ListIterator {
    fn technetium_type_name(&self) -> String {
        "iterator(list)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let mut index = self.index.lock().unwrap();
        if *index >= self.contents.len() {
            Ok(None)
        } else {
            let old = *index;
            *index += 1;
            Ok(Some(Arc::clone(&self.contents[old])))
        }
    }
}

pub struct Tuple {
    pub contents: Vec<ObjectRef>,
}

impl Object for Tuple {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let mut res_contents = vec![];
        for val in self.contents.iter() {
            res_contents.push(val.technetium_clone()?);
        }
        Ok(Arc::new(Tuple { contents: res_contents }))
    }

    fn technetium_type_name(&self) -> String {
        "tuple".to_string()
    }

    fn truthy(&self) -> bool {
        self.contents.len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
       match method {
            "length" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args".to_string()))
                } else {
                    Ok(IntObject::new(self.contents.len() as i64))
                }
            },
            _ => {
                Err(RuntimeError::type_error(format!("list has no method {}", method)))
            },
       }
    }
}

pub struct VoidObject;

impl VoidObject {
    pub fn new() -> ObjectRef {
        Arc::new(VoidObject)
    }
}

impl Object for VoidObject {
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
