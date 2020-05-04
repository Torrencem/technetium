
use std::collections::{HashMap, HashSet};
use crate::bytecode::*;
use crate::error::*;
use crate::core::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct MemoryManager {
    memory: HashMap<FrameId, HashMap<LocalName, ObjectRef>>,
    frame_index: HashMap<FrameId, ContextId>,
    do_not_drops: HashMap<ContextId, HashSet<LocalName>>,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager {
            memory: HashMap::new(),
            frame_index: HashMap::new(),
            do_not_drops: HashMap::new(),
        }
    }

    pub fn register_context(&mut self, cid: ContextId) {
        self.do_not_drops.insert(cid, HashSet::new());
    }

    pub fn do_not_drop(&mut self, cid: ContextId, index: LocalName) -> RuntimeResult<()> {
        debug!("Registered a do-not-drop, cid: {}, index: {}", cid, index);
        let mut dnd = self.do_not_drops.get_mut(&cid);
        if let Some(dnd) = dnd {
            dnd.insert(index);
            Ok(())
        } else {
            Err(RuntimeError::internal_error("Inserted a do-not-drop into a context that doesn't exist"))
        }
    }

    pub fn register_frame(&mut self, fid: FrameId, cid: ContextId) {
        self.memory.insert(fid, HashMap::new());
        self.frame_index.insert(fid, cid);
    }

    pub fn get(&self, index: NonLocalName) -> RuntimeResult<ObjectRef> {
        let frame = self.memory.get(&index.0).ok_or(RuntimeError::internal_error("Called get on a frame that doesn't exist"))?;

        let rc = frame.get(&index.1).ok_or(RuntimeError::internal_error("Called get on a value in a frame that doesn't exist"))?;

        Ok(Rc::clone(rc))
    }

    pub fn set(&mut self, index: NonLocalName, val: ObjectRef) -> RuntimeResult<()> {
        let mut frame = self.memory.get_mut(&index.0).ok_or(RuntimeError::internal_error("Called set on a frame that doesn't exist"))?;
        
        frame.insert(index.1, val);

        Ok(())
    }

    pub fn clear_frame(&mut self, fid: FrameId) -> RuntimeResult<()> {
        trace!("Clearing frame {}", fid);
        let mut frame = self.memory.get_mut(&fid).ok_or(RuntimeError::internal_error("Called clear frame on a frame that doesn't exist"))?;

        let context_id = self.frame_index.get(&fid).ok_or(RuntimeError::internal_error("Called clear frame on a frame that doesn't correspond to a context"))?;

        let dnd = self.do_not_drops.get(&context_id).ok_or(RuntimeError::internal_error("Called clear frame on an unregistered frame"))?;
        
        frame.retain(|index, _| {
            dnd.contains(index)
        });

        if frame.len() == 0 {
            drop(frame);
            self.memory.remove(&fid);
            self.frame_index.remove(&fid);
        }

        Ok(())
    }
}
