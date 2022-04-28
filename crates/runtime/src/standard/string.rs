use crate::error::*;
use crate::prelude::*;
use std::str;
use std::cell::Ref;

use ouroboros::self_referencing;

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
        let linesiterbuild = LinesIteratorBuilder {
            head: ObjectCell::clone(&this.parent),
            s_builder: |head| head.try_borrow().unwrap(),
            lines_builder: |s| s.val.lines()
        };

        Ok(ObjectRef::new(linesiterbuild.build()))
    }
}

// Rentals must be used because str::Lines takes a reference
// to a String, and we own the string it takes a reference to

#[self_referencing]
pub struct LinesIterator {
    head: ObjectCell<StringObject>,
    #[covariant]
    #[borrows(head)]
    s: Ref<'this, StringObject>,
    #[not_covariant]
    #[borrows(s)]
    lines: str::Lines<'this>,
}

impl Object for ObjectCell<LinesIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(lines)".to_string()
    }

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        this.with_mut(|fields| {
            let next = fields.lines.next().map(|val| val.to_string());
            Ok(next.map(StringObject::new))
        })
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

        let charsiterbuild =
            CharsIteratorBuilder {
                head: ObjectCell::clone(&this.parent),
                s_builder: |head| head.try_borrow().unwrap(),
                chars_builder: |s| s.val.chars()
            };

        Ok(ObjectRef::new(charsiterbuild.build()))
    }
}

#[self_referencing]
pub struct CharsIterator {
    head: ObjectCell<StringObject>,
    #[covariant]
    #[borrows(head)]
    s: Ref<'this, StringObject>,
    #[not_covariant]
    #[borrows(s)]
    chars: str::Chars<'this>,
}

impl Object for ObjectCell<CharsIterator> {
    fn technetium_type_name(&self) -> String {
        "iterator(chars)".to_string()
    }

    fn take_iter(&self, _context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let mut this = self.try_borrow_mut()?;
        this.with_mut(|fields| {
            let next = fields.chars.next();
            Ok(next.map(CharObject::new))
        })
    }
}
