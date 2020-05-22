
use crate::error::*;
use crate::prelude::*;

use crate::func_object;

#[derive(Clone)]
pub struct Map {
    parent_iter: ObjectRef,
    map_func: ObjectRef,
}

func_object!(MapFunc, (2..=2), _c, args -> {
    Ok(ObjectRef::new(Map {
        parent_iter: ObjectRef::clone(&args[0]),
        map_func: ObjectRef::clone(&args[1]),
    }))
});

impl Object for ObjectCell<Map> {
    fn technetium_clone(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;

        Ok(ObjectRef::new(Map {
            parent_iter: this.parent_iter.technetium_clone(context)?,
            map_func: this.map_func.technetium_clone(context)?,
        }))
    }

    fn technetium_type_name(&self) -> String {
        "map".to_string()
    }

    fn make_iter(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;
        Ok(ObjectRef::new(MapIterator {
            parent_iter: this.parent_iter.make_iter(context)?,
            map_func: ObjectRef::clone(&this.map_func),
        }))
    }
}

#[derive(Clone)]
pub struct MapIterator {
    parent_iter: ObjectRef,
    map_func: ObjectRef,
}

impl Object for ObjectCell<MapIterator> {
    fn technetium_clone(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<ObjectRef> {
        let this = self.try_borrow()?;

        Ok(ObjectRef::new(Map {
            parent_iter: this.parent_iter.technetium_clone(context)?,
            map_func: this.map_func.technetium_clone(context)?,
        }))
    }

    fn technetium_type_name(&self) -> String {
        "iterator(map)".to_string()
    }

    fn take_iter(&self, context: &mut RuntimeContext<'_>) -> RuntimeResult<Option<ObjectRef>> {
        let this = self.try_borrow()?;
        let val = this.parent_iter.take_iter(context)?;
        match val {
            Some(val) => Ok(Some(this.map_func.call(&[val], context)?)),
            None => Ok(None)
        }
    }
}
