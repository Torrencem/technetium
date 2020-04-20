
use std::sync::Arc;
use std::any::Any;
use std::any::TypeId;
use std::fmt;

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
    AttributeError
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
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Object : Any + ToAny {
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
}

impl fmt::Debug for dyn Object {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.to_string().unwrap_or("Object".to_string()))
    }
}

pub struct IntObject {
    val: i64,
}

impl IntObject {
    pub fn new(val: i64) -> ObjectRef {
        let res = Arc::new(IntObject { val: val });
        res
    }
}

impl Object for IntObject {
    fn rush_type_name(&self) -> String {
        "int".to_string()
    }

    fn to_string(&self) -> Result<String> {
        Ok(format!("{}", self.val).to_string())
    }
}

pub fn add(a: ObjectRef, b: ObjectRef) -> Result<ObjectRef> {
    let a_any = a.as_any();
    let b_any = b.as_any();
    match (a_any.type_id(), b_any.type_id()) {
        (a, b) if a == TypeId::of::<IntObject>() && b == TypeId::of::<IntObject>() => {
            let int_a = a_any.downcast_ref::<IntObject>().unwrap();
            let int_b = b_any.downcast_ref::<IntObject>().unwrap();
            let res = IntObject::new(int_a.val + int_b.val);
            Ok(res)
        },
        _ => {
            Err(RuntimeError::type_error(format!("Cannot add type {} to type {}", a.rush_type_name(), b.rush_type_name())))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let a = IntObject::new(5);
        let b = IntObject::new(10);
        assert!(add(a, b).is_ok());
    }
}
