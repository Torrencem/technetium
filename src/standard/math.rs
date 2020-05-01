
use crate::core::*;
use crate::builtins::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bytecode::NonLocalName;
use crate::bytecode::{ContextId, FrameId};
use crate::error::*;

use std::io::{self, Write};
use std::process::{Command, Child, Stdio, Output};

use std::f64;

use crate::func_object;

func_object!(Sin, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.sin()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).sin()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Cos, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.cos()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).cos()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Tan, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.tan()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).tan()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Abs, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.abs()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).abs()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Sqrt, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.sqrt()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).sqrt()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Exp, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.exp()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).exp()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Ln, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.ln()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).ln()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Arcsin, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.asin()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).asin()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Arccos, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.acos()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).acos()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Arctan, (1..=1), args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<FloatObject>() {
        Ok(FloatObject::new(float_obj.val.atan()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<IntObject>() {
        Ok(FloatObject::new((int_obj.val as f64).atan()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});
