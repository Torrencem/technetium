use crate::bytecode;
use crate::bytecode::Op;
use crate::bytecode::{ContextId, FrameId, NonLocalName};
use crate::builtins::index_get;
use crate::standard;
use std::any::Any;
use std::clone::Clone as RustClone;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

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

pub trait Object: Any + ToAny {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::type_error(format!(
            "{} can not be cloned",
            self.technetium_type_name()
        )))
    }

    fn technetium_type_name(&self) -> String;

    fn to_string(&self) -> RuntimeResult<String> {
        Ok(format!(
            "<{}>",
            self.technetium_type_name()
        ))
    }

    fn get_attr(&self, attr: String) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!(
            "{} has no attributes",
            self.technetium_type_name()
        )))
    }

    fn set_attr(&self, attr: String, val: ObjectRef) -> RuntimeResult<()> {
        Err(RuntimeError::attribute_error(format!(
            "Cannot set attributes of {}",
            self.technetium_type_name()
        )))
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        Err(RuntimeError::attribute_error(format!(
            "Cannot call method of {}",
            self.technetium_type_name()
        )))
    }

    fn call(
        &self,
        args: &[ObjectRef],
        locals: &mut HashMap<NonLocalName, ObjectRef>,
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
pub struct CharObject {
    pub val: char,
}

impl CharObject {
    pub fn new(c: char) -> ObjectRef {
        Arc::new(CharObject { val: c })
    }
}

impl Object for CharObject {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(self.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "char".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        Ok(self.val.to_string())
    }

    fn truthy(&self) -> bool {
        !self.val.is_whitespace()
    }
}

#[derive(Debug)]
pub struct StringObject {
    pub val: RwLock<String>,
}

impl StringObject {
    pub fn new(s: String) -> ObjectRef {
        Arc::new(StringObject { val: RwLock::new(s) })
    }
}

impl Object for StringObject {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let val = self.val.read()?;
        Ok(StringObject::new(val.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "string".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let val = self.val.read()?;
        Ok(val.clone())
    }

    fn truthy(&self) -> bool {
        let val = self.val.read().unwrap();
        *val != ""
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        match method {
            "length" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(self.val.read()?.len() as i64))
                }
            },
            "escape" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(StringObject::new(self.val.read()?.escape_default().collect()))
                }
            },
            "lines" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("lines expects 0 args"))
                } else {
                    Ok(Arc::new(standard::string::Lines {
                        parent: Arc::new(StringObject { val: RwLock::new(self.val.read()?.clone()) }),
                    }))
                }
            },
            _ => Err(RuntimeError::type_error(format!(
                "string has no method {}",
                method
            ))),
        }
    }
}

pub struct Function {
    pub nargs: usize,
    pub name: String,
    pub context: Arc<bytecode::GlobalContext>,
    pub code: Vec<Op>,
    pub context_id: ContextId,
    pub least_ancestors: RwLock<Option<HashMap<ContextId, FrameId>>>,
}

impl Object for Function {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(Function {
            nargs: self.nargs,
            name: self.name.clone(),
            context: Arc::clone(&self.context),
            code: self.code.clone(),
            context_id: self.context_id,
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
        locals: &mut HashMap<NonLocalName, ObjectRef>,
    ) -> RuntimeResult<ObjectRef> {
        if args.len() != self.nargs {
            return Err(RuntimeError::type_error(format!(
                "Incorrect number of arguments given to {}: expected {}, got {}",
                self.name,
                self.nargs,
                args.len()
            )));
        }
        let mut frame = bytecode::Frame::new(
            &self.code,
            locals,
            Arc::clone(&self.context),
            self.least_ancestors
                .read()?
                .as_ref()
                .unwrap()
                .clone(),
            self.context_id,
        );
        for arg in args.iter().rev() {
            frame.stack.push(Arc::clone(arg));
        }
        frame.run()
    }
}

pub struct List {
    pub contents: RwLock<Vec<ObjectRef>>,
}

impl Object for List {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let mut res_contents = vec![];
        let contents_ = self.contents.read()?;
        for val in contents_.iter() {
            res_contents.push(val.technetium_clone()?);
        }
        Ok(Arc::new(List {
            contents: RwLock::new(res_contents),
        }))
    }

    fn technetium_type_name(&self) -> String {
        "list".to_string()
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let mut res = String::new();
        res.push('[');
        let mut first = true;
        let vals = self.contents.read()?;
        for val in vals.iter() {
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
        self.contents.read().unwrap().len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> RuntimeResult<ObjectRef> {
        match method {
            "length" => {
                if args.len() > 0 {
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(self.contents.read()?.len() as i64))
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
            contents: self
                .contents
                .read()?
                .iter()
                .map(|val| Arc::clone(val))
                .collect(),
            index: RwLock::new(0),
        };

        Ok(Arc::new(iter))
    }
}

pub struct ListIterator {
    pub contents: Vec<ObjectRef>,
    pub index: RwLock<usize>,
}

impl Object for ListIterator {
    fn technetium_type_name(&self) -> String {
        "iterator(list)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let mut index = self.index.write()?;
        if *index >= self.contents.len() {
            Ok(None)
        } else {
            let old = *index;
            *index += 1;
            Ok(Some(Arc::clone(&self.contents[old])))
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

impl Object for Slice {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(Slice {
            parent: Arc::clone(&self.parent),
            start: self.start,
            stop: self.stop,
            step: self.step,
        }))
    }

    fn technetium_type_name(&self) -> String {
        "slice".to_string()
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(SliceIterator {
            parent: Arc::clone(&self.parent),
            curr: RwLock::new(self.start),
            stop: self.stop,
            step: self.step,
        }))
    }

    fn to_string(&self) -> RuntimeResult<String> {
        let mut res = String::new();
        res.push('[');
        let mut first = true;
        let mut curr_index = self.start;
        loop {
            if let Some(stop) = self.stop {
                if self.step < 0 && curr_index <= stop
                || self.step > 0 && curr_index >= stop {
                    break;
                }
            }
            let val = index_get(Arc::clone(&self.parent), IntObject::new(curr_index));
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
            curr_index += self.step;
        }
        res.push(']');
        Ok(res)
    }
}

#[derive(Debug)]
pub struct SliceIterator {
    pub parent: ObjectRef,
    pub curr: RwLock<i64>,
    pub stop: Option<i64>,
    pub step: i64,
}

impl Object for SliceIterator {
    fn technetium_type_name(&self) -> String {
        "iterator(slice)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let mut curr_ = self.curr.write()?;
        if let Some(stop) = self.stop {
            if self.step < 0 && *curr_ <= stop
            || self.step > 0 && *curr_ >= stop {
                return Ok(None);
            }
        }
        let old = index_get(Arc::clone(&self.parent), IntObject::new(*curr_));
        *curr_ += self.step;
        Ok(old.ok())
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
        Ok(Arc::new(Tuple {
            contents: res_contents,
        }))
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
                    Err(RuntimeError::type_error("length expects 0 args"))
                } else {
                    Ok(IntObject::new(self.contents.len() as i64))
                }
            }
            _ => Err(RuntimeError::type_error(format!(
                "list has no method {}",
                method
            ))),
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
