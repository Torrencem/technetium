use crate::bytecode::*;
use crate::error::*;
use crate::prelude::*;
use std::collections::HashSet;

pub trait BackingIndex {
    fn to_usize(&self) -> usize;
}

impl BackingIndex for u16 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl BackingIndex for u32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Clone)]
pub struct MemoryBacking<T: Clone> {
    backing: Vec<Option<T>>,
}

impl<T: Clone> MemoryBacking<T> {
    pub fn new() -> Self {
        MemoryBacking { backing: vec![] }
    }

    pub fn get<Num: BackingIndex>(&self, index: Num) -> Option<&T> {
        let index: usize = index.to_usize();
        if index >= self.backing.len() {
            None
        } else if let Some(r) = &self.backing[index] {
            Some(r)
        } else {
            None
        }
    }

    pub fn get_mut<Num: BackingIndex>(&mut self, index: Num) -> Option<&mut T> {
        let index: usize = index.to_usize();
        if index >= self.backing.len() {
            None
        } else if let Some(r) = &mut self.backing[index] {
            Some(r)
        } else {
            None
        }
    }

    pub fn insert<Num: BackingIndex>(&mut self, index: Num, val: T) {
        let index: usize = index.to_usize();
        if index >= self.backing.len() {
            self.backing
                .append(&mut vec![None; index + self.backing.len() + 1]);
        }
        self.backing[index] = Some(val);
    }

    pub fn remove<Num: BackingIndex>(&mut self, index: Num) {
        let index: usize = index.to_usize();
        self.backing[index] = None;
        // TODO: Clear empty memory?
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(usize) -> bool,
    {
        for i in 0..self.backing.len() {
            if !f(i) {
                self.backing[i] = None;
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut total = 0;
        for val in self.backing.iter() {
            if val.is_some() {
                total += 1;
            }
        }
        total
    }
}

#[derive(Debug)]
pub struct MemoryManager {
    memory: MemoryBacking<MemoryBacking<ObjectRef>>,
    frame_index: MemoryBacking<ContextId>,
    do_not_drops: MemoryBacking<HashSet<LocalName>>,
}

impl Default for memory::MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            memory: MemoryBacking::new(),
            frame_index: MemoryBacking::new(),
            do_not_drops: MemoryBacking::new(),
        }
    }

    pub fn register_context(&mut self, cid: ContextId) {
        self.do_not_drops.insert(cid, HashSet::new());
    }

    pub fn do_not_drop(&mut self, cid: ContextId, index: LocalName) -> RuntimeResult<()> {
        let dnd = self.do_not_drops.get_mut(cid);
        if let Some(dnd) = dnd {
            dnd.insert(index);
            Ok(())
        } else {
            Err(RuntimeError::internal_error(
                "Inserted a do-not-drop into a context that doesn't exist",
            ))
        }
    }

    pub fn register_frame(&mut self, fid: FrameId, cid: ContextId) {
        self.memory.insert(fid, MemoryBacking::new());
        self.frame_index.insert(fid, cid);
    }

    pub fn get(&self, index: NonLocalName) -> RuntimeResult<ObjectRef> {
        let frame = self.memory.get(index.0).ok_or_else(|| {
            RuntimeError::variable_undefined_error("Variable was not initialized")
        })?;

        let rc = frame.get(index.1).ok_or_else(|| {
            RuntimeError::variable_undefined_error("Variable was not initialized")
        })?;

        Ok(ObjectRef::clone(rc))
    }

    pub fn set(&mut self, index: NonLocalName, val: ObjectRef) -> RuntimeResult<()> {
        let frame = self.memory.get_mut(index.0).ok_or_else(|| {
            RuntimeError::internal_error("Called set on a frame that doesn't exist")
        })?;

        frame.insert(index.1, val);

        Ok(())
    }

    pub fn clear_frame(&mut self, fid: FrameId) -> RuntimeResult<()> {
        let frame = self.memory.get_mut(fid).ok_or_else(|| {
            RuntimeError::internal_error("Called clear frame on a frame that doesn't exist")
        })?;

        let context_id = self.frame_index.get(fid).ok_or_else(|| {
            RuntimeError::internal_error(
                "Called clear frame on a frame that doesn't correspond to a context",
            )
        })?;

        let dnd = self.do_not_drops.get(*context_id).ok_or_else(|| {
            RuntimeError::internal_error("Called clear frame on an unregistered frame")
        })?;

        frame.retain(|index| dnd.contains(&(index as u16)));

        if frame.len() == 0 {
            self.memory.remove(fid);
            self.frame_index.remove(fid);
        }

        Ok(())
    }
}
