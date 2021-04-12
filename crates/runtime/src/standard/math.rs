use crate::error::*;
use crate::prelude::*;

use std::f64;

use crate::func_object;
use rand::Rng;
use std::any::TypeId;

use rand_distr::{Normal, Distribution, StandardNormal};

func_object!(Sin, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.sin()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).sin()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Cos, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.cos()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).cos()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Tan, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.tan()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).tan()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Abs, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.abs()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).abs()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Sqrt, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.sqrt()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).sqrt()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Exp, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.exp()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).exp()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Ln, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.ln()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).ln()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Arcsin, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.asin()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).asin()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Arccos, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.acos()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).acos()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Arctan, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(float_obj) = arg_any.downcast_ref::<ObjectCell<FloatObject>>() {
        let float_obj = float_obj.try_borrow()?;
        Ok(FloatObject::new(float_obj.val.atan()))
    } else if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        Ok(FloatObject::new((int_obj.to_i64()? as f64).atan()))
    } else {
        Err(RuntimeError::type_error("Incorrect type as argument to sin; expected number"))
    }
});

func_object!(Rand, (0..=0), _c, args -> {
    let mut rng = rand::thread_rng();
    Ok(FloatObject::new(rng.gen_range(0.0..1.0)))
});

func_object!(RandRange, (2..=2), _c, args -> {
    let mut rng = rand::thread_rng();
    match_tech_types!((args[0], args[1]) {
        (val_a: IntObject, val_b: IntObject) => {
            Ok(IntObject::new(rng.gen_range(val_a.to_i64()?..val_b.to_i64()?)))
        },
        (val_a: FloatObject, val_b: FloatObject) => {
            Ok(FloatObject::new(rng.gen_range(val_a.val..val_b.val)))
        },
        (val_a: FloatObject, val_b: IntObject) => {
            Ok(FloatObject::new(rng.gen_range(val_a.val..val_b.to_i64()? as f64)))
        },
        (val_a: IntObject, val_b: FloatObject) => {
            Ok(FloatObject::new(rng.gen_range(val_a.to_i64()? as f64..val_b.val)))
        },
        _ => {
            Err(RuntimeError::type_error("Incorrect type as argument to rand_range; expected both numerical types"))
        }
    })
});

func_object!(RandNormal, (0..=2), _c, args -> {
    let mean = {
        if args.len() == 0 {
            0.0
        } else {
            downcast!((val_a: FloatObject = args[0]) -> {
                val_a.val
            } else {
                downcast!((val_a: IntObject = args[0]) -> {
                    val_a.to_i64()? as f64
                } else {
                    return Err(RuntimeError::type_error("Incorrect type as argument to rand_normal; expected numerical types"));
                })
            })
        }
    };
    let stdev = {
        if args.len() < 2 {
            1.0
        } else {
            downcast!((val_b: FloatObject = args[1]) -> {
                val_b.val
            } else {
                downcast!((val_b: IntObject = args[1]) -> {
                    val_b.to_i64()? as f64
                } else {
                    return Err(RuntimeError::type_error("Incorrect type as argument to rand_normal; expected numerical types"));
                })
            })
        }
    };
    let mut rng = rand::thread_rng();
    if mean == 0.0 && stdev == 1.0 {
        Ok(FloatObject::new(rng.sample(StandardNormal)))
    } else {
        let normal = Normal::new(mean, stdev).map_err(|_| RuntimeError::type_error("Invalid mean and standard deviation given to rand_normal"))?;
        Ok(FloatObject::new(normal.sample(&mut rng)))
    }
});
