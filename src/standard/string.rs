use crate::core::*;
use crate::error::*;
use std::sync::RwLock;
use std::rc::Rc;
use std::str;

use rental::rental;

#[derive(Debug, Clone)]
pub struct Lines {
    pub parent: Rc<StringObject>,
}

impl Object for Lines {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        Ok(Rc::new(Lines { parent: Rc::clone(&self.parent) }))
    }
    
    fn technetium_type_name(&self) -> String {
        "lines".to_string()
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        let lines_iter_rental_head = rentals::LinesIteratorHead::new(
            Rc::clone(&self.parent),
            |rc| rc.val.read().unwrap()
        );

        Ok(Rc::new(
                LinesIterator {
                    inner: RwLock::new(rentals::LinesIterator::new(
                            Box::new(lines_iter_rental_head),
                            |head| head.parent.lines()))
                }
        ))
    }
}

// Rentals must be used because str::Lines takes a reference
// to a String, and we own the string it takes a reference to
pub struct LinesIterator {
    pub inner: RwLock<rentals::LinesIterator>,
}

rental! {
    mod rentals {
        use super::*;
        use std::sync;
        
        #[rental]
        pub struct LinesIteratorHead {
            head: Rc<StringObject>,
            parent: sync::RwLockReadGuard<'head, String>,
        }

        #[rental]
        pub struct LinesIterator {
            #[subrental = 2]
            head: Box<LinesIteratorHead>,
            lines: str::Lines<'head_1>,
        }
    }
}

impl Object for LinesIterator {
    fn technetium_type_name(&self) -> String {
        "iterator(lines)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let mut inner = self.inner.write()?;
        let next = rentals::LinesIterator::rent_mut(&mut inner, |lines| lines.next().map(|val| val.to_string()));
        Ok(next.map(|s| StringObject::new(s.to_string())))
    }
}
