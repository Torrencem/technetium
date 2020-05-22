use crate::error::*;
use crate::prelude::*;
use std::rc::Rc;
use std::str;

use rental::rental;

#[derive(Debug, Clone)]
pub struct Lines {
    pub parent: ObjectCell<StringObject>,
}

impl Object for ObjectCell<Lines> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(Lines {
            parent: ObjectCell::clone(&this.parent),
        }))
    }

    fn technetium_type_name(&self) -> String {
        "lines".to_string()
    }

    fn make_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let lines_iter_rental_head =
            line_rentals::LinesIteratorHead::new(ObjectCell::clone(&this.parent), |rc| rc.borrow());

        Ok(ObjectRef::new(LinesIterator {
            inner: line_rentals::LinesIterator::new(Box::new(lines_iter_rental_head), |head| {
                head.parent.val.lines()
            }),
        }))
    }
}

// Rentals must be used because str::Lines takes a reference
// to a String, and we own the string it takes a reference to
pub struct LinesIterator {
    pub inner: line_rentals::LinesIterator,
}

rental! {
    mod line_rentals {
        use super::*;
        use std::cell::Ref;
        use mlrefcell::MLRefCell;

        #[rental]
        pub struct LinesIteratorHead {
            #[target_ty = "Rc<MLRefCell<StringObject>>"]
            head: ObjectCell<StringObject>,
            parent: Ref<'head, StringObject>,
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

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        let inner = &mut this.inner;
        let next = line_rentals::LinesIterator::rent_mut(inner, |lines| {
            lines.next().map(|val| val.to_string())
        });
        Ok(next.map(StringObject::new))
    }
}

#[derive(Debug, Clone)]
pub struct Chars {
    pub parent: ObjectCell<StringObject>,
}

impl Object for ObjectCell<Chars> {
    fn technetium_clone(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(Chars {
            parent: ObjectCell::clone(&this.parent),
        }))
    }

    fn technetium_type_name(&self) -> String {
        "chars".to_string()
    }

    fn make_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        let lines_iter_rental_head =
            char_rentals::CharsIteratorHead::new(ObjectCell::clone(&this.parent), |rc| rc.borrow());

        Ok(ObjectRef::new(CharsIterator {
            inner: char_rentals::CharsIterator::new(Box::new(lines_iter_rental_head), |head| {
                head.parent.val.chars()
            }),
        }))
    }
}

pub struct CharsIterator {
    pub inner: char_rentals::CharsIterator,
}

rental! {
    mod char_rentals {
        use super::*;
        use std::cell::Ref;
        use mlrefcell::MLRefCell;

        #[rental]
        pub struct CharsIteratorHead {
            #[target_ty = "Rc<MLRefCell<StringObject>>"]
            head: ObjectCell<StringObject>,
            parent: Ref<'head, StringObject>,
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

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        let inner = &mut this.inner;
        let next = char_rentals::CharsIterator::rent_mut(inner, |lines| lines.next());
        Ok(next.map(CharObject::new))
    }
}
