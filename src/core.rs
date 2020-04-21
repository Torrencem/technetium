
use std::sync::Arc;
use std::any::Any;
use std::fmt;
use std::clone::Clone as RustClone;

pub type ObjectRef = Arc<dyn Object>;

pub trait ToAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Object> ToAny for T {
    fn as_any(&self) -> &dyn Any {
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
}

impl RuntimeError {
    pub fn type_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::TypeError,
            help: message,
        }
    }
    
    pub fn attribute_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::AttributeError,
            help: message
        }
    }

    pub fn internal_error(message: String) -> Self {
        RuntimeError {
            err: ErrorType::InternalError,
            help: message
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Object : Any + ToAny {
    fn clone(&self) -> Result<ObjectRef>;

    fn rush_type_name(&self) -> String;

    fn to_string(&self) -> Result<String> {
        Err(RuntimeError::type_error(format!("{} can not be converted into a string", self.rush_type_name())))
    }

    fn get_attr(&self, attr: String) -> Result<ObjectRef> {
        Err(RuntimeError::attribute_error(format!("{} has no attributes", self.rush_type_name())))
    }

    fn set_attr(&self, attr: String, val: ObjectRef) -> Result<()> {
        Err(RuntimeError::attribute_error(format!("Cannot set attributes of {}", self.rush_type_name())))
    }

    fn call_method(&self, method: &str, args: &[ObjectRef]) -> Result<ObjectRef> {
        Err(RuntimeError::attribute_error(format!("Cannot call method of {}", self.rush_type_name())))
    }

    fn call(&self, args: &[ObjectRef]) -> Result<ObjectRef> {
        Err(RuntimeError::type_error(format!("Object of type {} is not callable", self.rush_type_name())))
    }

    fn truthy(&self) -> bool;
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
    fn clone(&self) -> Result<ObjectRef> {
        Ok(BoolObject::new(self.val))
    }

    fn rush_type_name(&self) -> String {
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
    fn clone(&self) -> Result<ObjectRef> {
        Ok(IntObject::new(self.val))
    }

    fn rush_type_name(&self) -> String {
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
    fn clone(&self) -> Result<ObjectRef> {
        Ok(FloatObject::new(self.val))
    }

    fn rush_type_name(&self) -> String {
        "int".to_string()
    }

    fn to_string(&self) -> Result<String> {
        Ok(format!("{}", self.val))
    }

    fn truthy(&self) -> bool {
        self.val != 0.0
    }
}


impl Object for String {
    fn clone(&self) -> Result<ObjectRef> {
        Ok(Arc::new(RustClone::clone(self)))
    }

    fn rush_type_name(&self) -> String {
        "string".to_string()
    }

    fn to_string(&self) -> Result<String> {
        Ok(RustClone::clone(self))
    }

    fn truthy(&self) -> bool {
        self != ""
    }
}

