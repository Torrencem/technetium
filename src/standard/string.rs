use crate::core::*;
use crate::error::*;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Lines {
    pub parent: Arc<StringObject>,
}

impl Object for Lines {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(Lines { parent: Arc::clone(&self.parent) }))
    }
    
    fn technetium_type_name(&self) -> String {
        "lines".to_string()
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        Ok(Arc::new(LinesIterator { parent: Arc::clone(&self.parent), current_index: Mutex::new(0), done: Mutex::new(false) }))
    }
}

#[derive(Debug)]
pub struct LinesIterator {
    pub parent: Arc<StringObject>,
    pub current_index: Mutex<usize>,
    pub done: Mutex<bool>
}

impl Object for LinesIterator {
    fn technetium_type_name(&self) -> String {
        "iterator(lines)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        if *self.done.lock()? {
            return Ok(None);
        }
        let mut res = String::new();
        let s = self.parent.val.lock()?;
        let mut curr = self.current_index.lock()?;

        loop {
            match s.chars().nth(*curr) {
                None => {
                    *self.done.lock()? = true;
                    return Ok(Some(StringObject::new(res)));
                }
                Some('\n') => {
                    if s.chars().nth(*curr + 1) == Some('\r') {
                        *curr += 2;
                    } else {
                        *curr += 1;
                    }
                    if s.chars().nth(*curr) == None {
                        *self.done.lock()? = true;
                    } 
                    return Ok(Some(StringObject::new(res)));
                }
                Some(c) => {
                    res.push(c);
                    *curr += 1;
                }
            }
        }
    }
}
