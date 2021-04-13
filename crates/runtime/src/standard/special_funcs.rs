use crate::error::*;
use crate::prelude::*;

use crate::{func_object, func_object_void};

use std::process::exit;
use std::time::SystemTime;
use std::fs;
use std::collections::HashMap;

use num::bigint::ToBigInt;

use std::io::{self, Write};

use glob::glob;
use std::result::Result;

func_object_void!(Print, (0..), _c, args -> {
    let mut first = true;
    for arg in args.iter() {
        if !first {
            print!("\t");
        } else {
            first = false;
        }
        print!("{}", arg.to_string()?);
    }
    io::stdout().flush()?;
});

func_object_void!(Printr, (0..), _c, args -> {
    let mut first = true;
    for arg in args.iter() {
        if !first {
            print!("\t");
        } else {
            first = false;
        }
        print!("{}", arg.to_string()?);
    }
    print!("\r");
    io::stdout().flush()?;
});

func_object_void!(Println, (0..), _c, args -> {
    let mut first = true;
    for arg in args.iter() {
        if !first {
            print!("\t");
        } else {
            first = false;
        }
        print!("{}", arg.to_string()?);
    }
    println!();
});

func_object_void!(Eprint, (0..), _c, args -> {
    let mut first = true;
    for arg in args.iter() {
        if !first {
            eprint!("\t");
        } else {
            first = false;
        }
        eprint!("{}", arg.to_string()?);
    }
    io::stdout().flush()?;
});

func_object_void!(Eprintr, (0..), _c, args -> {
    let mut first = true;
    for arg in args.iter() {
        if !first {
            eprint!("\t");
        } else {
            first = false;
        }
        eprint!("{}", arg.to_string()?);
    }
    eprint!("\r");
    io::stdout().flush()?;
});

func_object_void!(Eprintln, (0..), _c, args -> {
    let mut first = true;
    for arg in args.iter() {
        if !first {
            eprint!("\t");
        } else {
            first = false;
        }
        eprint!("{}", arg.to_string()?);
    }
    eprintln!();
});

func_object!(Exit, (1..=1), _c, args -> {
    let arg_any = args[0].as_any();
    if let Some(int_obj) = arg_any.downcast_ref::<ObjectCell<IntObject>>() {
        let int_obj = int_obj.try_borrow()?;
        exit(int_obj.to_i64()? as i32)
    } else {
        exit(if args[0].truthy() { 1 } else { 0 })
    }
});

func_object!(Type, (1..=1), _c, args -> {
    Ok(StringObject::new(args[0].technetium_type_name()))
});

func_object!(Hash, (1..=1), _c, args -> {
    let hash = args[0].technetium_hash().ok_or_else(|| RuntimeError::type_error(format!("Unhashable type: {}", args[0].technetium_type_name())))?;
    let hash = hash.to_bigint().unwrap();
    Ok(IntObject::new_big(hash))
});

func_object_void!(Lock, (1..=1), _c, args -> {
    args[0].lock_immutable()
});

func_object!(Clone_, (1..=1), context, args -> {
    Ok(args[0].technetium_clone(context)?)
});

func_object!(Assert, (1..=2), _c, args -> {
    if !args[0].truthy() {
        let message = if let Some(val) = args.get(1) {
            Some(val.to_string()?)
        } else {
            None
        };
        Err(RuntimeError::assertion_error(message))
    } else {
        Ok(UnitObject::new())
    }
});

func_object!(Version, (0..=0), _c, args -> {
    let (major, minor, patch) = (env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
    
    Ok(ObjectRef::new(Tuple {
        contents: vec![
            IntObject::new(major.parse::<i64>().unwrap()),
            IntObject::new(minor.parse::<i64>().unwrap()),
            IntObject::new(patch.parse::<i64>().unwrap()),
        ]
    }))
});

#[derive(Debug, Clone)]
pub struct Range {
    start: i64,
    end: i64,
    step: i64,
}

impl Object for ObjectCell<Range> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(this.clone()))
    }

    fn technetium_type_name(&self) -> String {
        "range".to_string()
    }

    fn make_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(RangeIterator::new(this.clone()))
    }
}

pub struct RangeIterator {
    inner: Range,
    curr: i64,
}

impl RangeIterator {
    pub fn new(inner: Range) -> ObjectRef {
        ObjectRef::new(RangeIterator {
            curr: inner.start,
            inner,
        })
    }
}

impl Object for ObjectCell<RangeIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(range)".to_string()
    }

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        let step = this.inner.step;
        let end = this.inner.end;
        let _curr = &mut this.curr;
        if (step < 0 && *_curr <= end) || (step > 0 && *_curr >= end) {
            return Ok(None);
        }
        let old = *_curr;
        *_curr += step;
        Ok(Some(IntObject::new(old)))
    }
}

func_object!(RangeFunc, (1..=3), _c, args -> {
    if args.len() == 1 {
        downcast!((int_obj: IntObject = args[0]) -> {
            Ok(ObjectRef::new(Range {
                start: 0,
                end: int_obj.to_i64()?,
                step: 1,
            }))
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range"))
        })
    } else if args.len() == 2 {
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<ObjectCell<IntObject>>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<ObjectCell<IntObject>>() {
                let int_obj_a = int_obj_a.try_borrow()?;
                let int_obj_b = int_obj_b.try_borrow()?;
                Ok(ObjectRef::new(Range {
                    start: int_obj_a.to_i64()?,
                    end: int_obj_b.to_i64()?,
                    step: 1,
                }))
            } else {
                Err(RuntimeError::type_error("Expected integer arguments to range"))
            }
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range"))
        }
    } else {
        if let Some(int_obj_a) = args[0].as_any().downcast_ref::<ObjectCell<IntObject>>() {
            if let Some(int_obj_b) = args[1].as_any().downcast_ref::<ObjectCell<IntObject>>() {
                if let Some(int_obj_c) = args[2].as_any().downcast_ref::<ObjectCell<IntObject>>() {
                    let int_obj_a = int_obj_a.try_borrow()?;
                    let int_obj_b = int_obj_b.try_borrow()?;
                    let int_obj_c = int_obj_c.try_borrow()?;
                    Ok(ObjectRef::new(Range {
                        start: int_obj_a.to_i64()?,
                        end: int_obj_b.to_i64()?,
                        step: int_obj_c.to_i64()?,
                    }))
                } else {
                    Err(RuntimeError::type_error("Expected integer arguments to range"))
                }
            } else {
                Err(RuntimeError::type_error("Expected integer arguments to range"))
            }
        } else {
            Err(RuntimeError::type_error("Expected integer arguments to range"))
        }
    }
});

// Staleness is a special feature that replicates the features of most build systems like make,
// where you can check if a file has been modified since the last time the build script was run.
// This feature needs to make use of a cache that can keep track of last modified times. For now,
// this information will be stored in a serialized HashMap with canonical paths in
// [script_parent_directory]/.tcmake/stale.cache
func_object!(Stale, (1..), _c, args -> {
    let mut cache_location = crate::get_tcmake_dir().unwrap();
    cache_location.push("stale.cache");
    let previous_timestamps: Option<HashMap<PathBuf, SystemTime>> = {
        if !cache_location.exists() {
            None
        } else {
            let cache = fs::read(&cache_location);
            if let Ok(file) = cache {
                bincode::deserialize(&file).ok()
            } else {
                // info!("Error reading cache location: {:?}", e.err().unwrap());
                warn!("Error reading cache location to find previous time stamps for stale(): {:?}", cache.err().unwrap());
                None
            }
        }
    };
    let old_timestamps = previous_timestamps.unwrap_or_else(|| HashMap::new());
    // Files to check passed by the user. Might be expanded from globs, might include directories,
    // and could be passed in either from all arguments, or might be passed in as a list
    // Should all be canonicalized!
    let file_checks_raw: Vec<String> = {
        if args.len() == 1 {
            if let Some(list_obj) = args[0].as_any().downcast_ref::<ObjectCell<List>>() {
                let mut res = vec![];
                let list_obj = list_obj.try_borrow()?;
                let list = &list_obj.contents;
                for inner_obj in list.iter() {
                    if let Some(string_obj) = inner_obj.as_any().downcast_ref::<ObjectCell<StringObject>>() {
                        let string_obj = string_obj.try_borrow()?;
                        let string = &string_obj.val;
                        res.push(string.clone());
                    } else {
                        return Err(RuntimeError::type_error("Got a list containing a {:?} as an argument to stale(). Most likely expected a list of strings."));
                    }
                }
                res
            } else if let Some(string_obj) = args[0].as_any().downcast_ref::<ObjectCell<StringObject>>() {
                let string_obj = string_obj.try_borrow()?;
                let string = &string_obj.val;
                vec![string.clone()]
            } else {
                return Err(RuntimeError::type_error("Expected either a string or a list to be passed to stale()."));
            }
        } else {
            let mut res = vec![];
            for arg in args.iter() {
                if let Some(string_obj) = arg.as_any().downcast_ref::<ObjectCell<StringObject>>() {
                    let string_obj = string_obj.try_borrow()?;
                    let string = &string_obj.val;
                    res.push(string.clone())
                } else {
                    return Err(RuntimeError::type_error("Expected either a string or a list to be passed to stale()."));
                }
            }
            res
        }
    };
    let mut file_checks: Vec<PathBuf> = vec![];
    // Push the current script path, if it exists
    match crate::CURR_SCRIPT_PATH.get() {
        Some(path) => {
            file_checks.push(path.clone());
        },
        None => { }
    }

    for file in file_checks_raw.iter() {
        let mut ct = 0;
        for string in glob(file)
            .map_err(|e| RuntimeError::type_error(format!("Invalid or unknown file or pattern in call to stale(): {:?}", e).to_string()))?
            .filter_map(Result::ok) {
            let p = PathBuf::from(string).canonicalize();
            if p.is_err() {
                warn!("Error canonicalizing path ({:?}) in stale(). Will return true anyway.", p);
                return Ok(BoolObject::new(true));
            }
            file_checks.push(p.unwrap());
            ct += 1;
        }
        if ct == 0 {
            let mut e = RuntimeError::type_error(format!("File or pattern does not exist that was passed to stale(): {:?}", file).to_string());
            e.err = RuntimeErrorType::IOError;
            return Err(e);
        }
    }

    let mut new_timestamps = old_timestamps.clone();
    let mut changed_timestamps = false;

    for file in file_checks.iter() {
        let res = fs::metadata(file);
        if res.is_err() {
            warn!("Error reading metadata from file ({:?}) in stale(). Will return true, and update the files it can.", file);
            changed_timestamps = true;
            continue;
        }
        let modified = res.unwrap().modified();
        if modified.is_err() {
            warn!("Error reading \"modified\" metadata from file ({:?}) in stale(). Will return true, and update the files it can.", file);
            changed_timestamps = true;
            continue;
        }
        let modified = modified.unwrap();
        if old_timestamps.contains_key(file) {
            let old_modified = old_timestamps.get(file).unwrap();
            if old_modified < &modified {
                new_timestamps.insert(file.clone(), modified);
                changed_timestamps = true;
            }
        } else {
            new_timestamps.insert(file.clone(), modified);
            changed_timestamps = true;
        }
    }

    // Write out the cache again
    if changed_timestamps {
        // It kind of makes sense to ignore failure to write out to the cache. This just means
        // stale will always return true if there's some weird error.
        let e = fs::write(cache_location, &bincode::serialize(&new_timestamps).unwrap());
        if e.is_err() {
            warn!("Error writing to cache location: {:?}. Won't update cache, but will return true anyway.", e);
        }
        Ok(BoolObject::new(true))
    } else {
        Ok(BoolObject::new(false))
    }
});
