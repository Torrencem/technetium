use crate::core::*;
use crate::error::*;
use parking_lot::RwLock;
use std::rc::Rc;
use std::str;

use rental::rental;
use parking_lot::RwLockReadGuard;

#[derive(Debug, Clone)]
pub struct Lines {
    pub parent: Rc<StringObject>,
}

impl Object for ObjectCell<Lines> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(Lines { parent: Rc::clone(&this.parent) }))
    }
    
    fn technetium_type_name(&self) -> String {
        "lines".to_string()
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let lines_iter_rental_head = line_rentals::LinesIteratorHead::new(
            Rc::clone(&this.parent),
            |rc| rc.val.read()
        );

        Ok(ObjectRef::new(
                LinesIterator {
                    inner: RwLock::new(line_rentals::LinesIterator::new(
                            Box::new(lines_iter_rental_head),
                            |head| head.parent.lines()))
                }
        ))
    }
}

// Rentals must be used because str::Lines takes a reference
// to a String, and we own the string it takes a reference to
pub struct LinesIterator {
    pub inner: RwLock<line_rentals::LinesIterator>,
}

rental! {
    mod line_rentals {
        use super::*;
        
        #[rental]
        pub struct LinesIteratorHead {
            head: Rc<StringObject>,
            parent: RwLockReadGuard<'head, String>,
        }

        #[rental]
        pub struct LinesIterator {
            #[subrental = 2]
            head: Box<LinesIteratorHead>,
            lines: str::Lines<'head_1>,
        }
    }
}

impl Object for ObjectCell<LinesIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(lines)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let this = self.try_borrow()?;
        let mut inner = this.inner.write();
        let next = line_rentals::LinesIterator::rent_mut(&mut inner, |lines| lines.next().map(|val| val.to_string()));
        Ok(next.map(|s| StringObject::new(s.to_string())))
    }
}

#[derive(Debug, Clone)]
pub struct Chars {
    pub parent: Rc<StringObject>,
}

impl Object for ObjectCell<Chars> {
    fn technetium_clone(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(Chars { parent: Rc::clone(&this.parent) }))
    }
    
    fn technetium_type_name(&self) -> String {
        "chars".to_string()
    }

    fn make_iter(&self) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let lines_iter_rental_head = char_rentals::CharsIteratorHead::new(
            Rc::clone(&this.parent),
            |rc| rc.val.read()
        );

        Ok(ObjectRef::new(
                CharsIterator {
                    inner: RwLock::new(char_rentals::CharsIterator::new(
                            Box::new(lines_iter_rental_head),
                            |head| head.parent.chars()))
                }
        ))
    }
}

pub struct CharsIterator {
    pub inner: RwLock<char_rentals::CharsIterator>,
}

rental! {
    mod char_rentals {
        use super::*;
        
        #[rental]
        pub struct CharsIteratorHead {
            head: Rc<StringObject>,
            parent: RwLockReadGuard<'head, String>,
        }

        #[rental]
        pub struct CharsIterator {
            #[subrental = 2]
            head: Box<CharsIteratorHead>,
            lines: str::Chars<'head_1>,
        }
    }
}

impl Object for ObjectCell<CharsIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(chars)".to_string()
    }

    fn take_iter(&self) -> RuntimeResult<Option<ObjectRef>> {
        let this = self.try_borrow()?;
        let mut inner = this.inner.write();
        let next = char_rentals::CharsIterator::rent_mut(&mut inner, |lines| lines.next());
        Ok(next.map(|s| CharObject::new(s)))
    }
}
