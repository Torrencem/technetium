
use std::sync::Arc;
use std::any::Any;
use std::fmt;
use std::clone::Clone as RustClone;
use crate::bytecode::Op;
use crate::bytecode;
use crate::bytecode::{NonLocalName, ContextId, FrameId};
use std::sync::Mutex;
use codespan::{Span, FileId};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::collections::HashMap;

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

// Similar to https://stackoverflow.com/a/28664881/6504760
// Don't use this abomination unless absolutely necessary
// pub type ObjectAny = Arc<dyn Any>;
// pub fn ref_to_any(val: ObjectRef) -> ObjectAny {
//     unsafe {
//         let val = (Arc::into_raw(val).as_ref().unwrap()).as_any();
//         Arc::new(val as *const dyn Any)
//     }
// }

#[derive(Clone, Debug)]
pub struct RuntimeError {
    err: ErrorType,
    help: String,
    span: Option<Span>,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}: {}", self.err, self.help)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ErrorType {
    TypeError,
    AttributeError,
    InternalError,
    IndexOutOfBounds,
    ChildProcessError,
    IOError,
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError {
            err: ErrorType::IOError,
            help: error.to_string(),
            span: None,
        }
    }
}

impl RuntimeError {
    pub fn type_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::TypeError,
            help: message,
            span: None,
        }
    }
    
    pub fn attribute_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::AttributeError,
            help: message,
            span: None,
        }
    }

    pub fn internal_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::InternalError,
            help: message,
            span: None,
        }
    }

    pub fn index_oob_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::IndexOutOfBounds,
            help: message,
            span: None,
        }
    }
    
    pub fn child_process_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::ChildProcessError,
            help: message,
            span: None,
        }
    }

    pub fn attach_span(self, span: Span) -> Self {
        RuntimeError {
            err: self.err,
            help: self.help,
            span: Some(span),
        }
    }

    pub fn weak_attach_span(self, span: Span) -> Self {
        match self.span {
            Some(val) => {
                RuntimeError {
                    err: self.err,
                    help: self.help,
                    span: Some(val),
                }
            },
            None => {
                RuntimeError {
                    err: self.err,
                    help: self.help,
                    span: Some(span),
                }
            }
        }
    }
    
    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self.span {
            Some(span) => Diagnostic::error()
                .with_message(format!("Runtime Error: {:?}", self.err))
                .with_labels(vec![
                    Label::primary(fileid, span).with_message(&self.help),
                ]),
            None => Diagnostic::error()
                .with_message(&self.help),
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Object : Any + ToAny + Send + Sync {
    fn marsh_clone(&self) -> Result<ObjectRef> {
        Err(RuntimeError::type_error(format!("{} can not be cloned", self.marsh_type_name())))
    }

    fn marsh_type_name(&self) -> String;

    fn to_string(&self) -> Result<String> {
        Err(RuntimeError::type_error(format!("{} can not be converted into a string", self.marsh_type_name())))
    }

    fn get_attr(&self, attr: String) -> Result<ObjectRef> {
        Err(RuntimeError::attribute_error(format!("{} has no attributes", self.marsh_type_name())))
    }

    fn set_attr(&self, attr: String, val: ObjectRef) -> Result<()> {
        Err(RuntimeError::attribute_error(format!("Cannot set attributes of {}", self.marsh_type_name())))
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> Result<ObjectRef> {
        Err(RuntimeError::attribute_error(format!("Cannot call method of {}", self.marsh_type_name())))
    }

    fn call(&self, args: &[ObjectRef], locals: &mut HashMap<NonLocalName, ObjectRef>) -> Result<ObjectRef> {
        Err(RuntimeError::type_error(format!("Object of type {} is not callable", self.marsh_type_name())))
    }

    fn make_iter(&self) -> Result<ObjectRef> {
        Err(RuntimeError::type_error(format!("Object of type {} cannot be made into an iterator", self.marsh_type_name())))
    }
    
    fn take_iter(&self) -> Result<Option<ObjectRef>> {
        Err(RuntimeError::type_error(format!("Object of type {} cannot be iterated", self.marsh_type_name())))
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
        Arc::new(BoolObject { val: val })
    }
}

impl Object for BoolObject {
    fn marsh_clone(&self) -> Result<ObjectRef> {
        Ok(BoolObject::new(self.val))
    }

    fn marsh_type_name(&self) -> String {
        "boolean".to_string()
    }

    fn to_string(&self) -> Result<String> {
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
        let res = Arc::new(IntObject { val: val });
        res
    }
}

impl Object for IntObject {
    fn marsh_clone(&self) -> Result<ObjectRef> {
        Ok(IntObject::new(self.val))
    }

    fn marsh_type_name(&self) -> String {
        "int".to_string()
    }

    fn to_string(&self) -> Result<String> {
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
        let res = Arc::new(FloatObject { val: val });
        res
    }
}

impl Object for FloatObject {
    fn marsh_clone(&self) -> Result<ObjectRef> {
        Ok(FloatObject::new(self.val))
    }

    fn marsh_type_name(&self) -> String {
        "int".to_string()
    }

    fn to_string(&self) -> Result<String> {
        Ok(format!("{}", self.val))
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
    fn marsh_clone(&self) -> Result<ObjectRef> {
        Ok(Arc::new(RustClone::clone(self)))
    }

    fn marsh_type_name(&self) -> String {
        "string".to_string()
    }

    fn to_string(&self) -> Result<String> {
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
    fn marsh_clone(&self) -> Result<ObjectRef> {
        Ok(Arc::new(Function {
            nargs: self.nargs,
            name: self.name.clone(),
            context: Arc::clone(&self.context),
            code: self.code.clone(),
            context_id: self.context_id,
            least_ancestors: Mutex::new(None),
        }))
    }

    fn marsh_type_name(&self) -> String {
        "function".to_string()
    }

    fn truthy(&self) -> bool {
        true
    }
    
    fn call(&self, args: &[ObjectRef], locals: &mut HashMap<NonLocalName, ObjectRef>) -> Result<ObjectRef> {
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
    fn marsh_clone(&self) -> Result<ObjectRef> {
        let mut res_contents = vec![];
        let contents_ = self.contents.lock().unwrap();
        for val in contents_.iter() {
            res_contents.push(val.marsh_clone()?);
        }
        Ok(Arc::new(List { contents: Mutex::new(res_contents) }))
    }

    fn marsh_type_name(&self) -> String {
        "list".to_string()
    }

    fn truthy(&self) -> bool {
        self.contents.lock().unwrap().len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> Result<ObjectRef> {
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

    fn make_iter(&self) -> Result<ObjectRef> {
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
    fn marsh_type_name(&self) -> String {
        "iterator(list)".to_string()
    }

    fn take_iter(&self) -> Result<Option<ObjectRef>> {
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
    fn marsh_clone(&self) -> Result<ObjectRef> {
        let mut res_contents = vec![];
        for val in self.contents.iter() {
            res_contents.push(val.marsh_clone()?);
        }
        Ok(Arc::new(Tuple { contents: res_contents }))
    }

    fn marsh_type_name(&self) -> String {
        "tuple".to_string()
    }

    fn truthy(&self) -> bool {
        self.contents.len() != 0
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> Result<ObjectRef> {
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
    fn marsh_type_name(&self) -> String {
        "void".to_string()
    }

    fn to_string(&self) -> Result<String> {
        Ok("void".to_string())
    }

    fn truthy(&self) -> bool {
        false
    }
}
