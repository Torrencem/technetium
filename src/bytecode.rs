
use std::collections::HashMap;

use crate::core::*;

use std::sync::{Mutex, Arc};

use std::clone::Clone as RustClone;
use std::process::Command;

use crate::builtins;
use crate::standard::Default_Namespace;

use std::fmt;

use codespan::Span;

/// Unique to each instance of a function
pub type FrameId = u32;

pub struct FrameIdGen {
    last: FrameId,
}

lazy_static! {
    pub static ref FRAME_ID_GEN: Mutex<FrameIdGen> = {
        Mutex::new(FrameIdGen { last: 100 })
    };
}

pub fn gen_frame_id() -> FrameId {
    let mut gen = FRAME_ID_GEN.lock().unwrap();
    let old = gen.last;
    gen.last += 1;
    old
}

/// Unique to each function in source code
pub type ContextId = u16;
pub type LocalName = u16;
pub type NonLocalUnmappedName = (ContextId, u16);
pub type NonLocalName = (FrameId, u16);
pub type GlobalConstantDescriptor = (ContextId, u16);
pub type DebugSpanDescriptor = u16;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// An operation in the marsh virtual machine
pub enum Op {
    /// Do nothing
    nop,
    
    /// Store the object on the top of the stack in a local variable
    store(LocalName),
    store_non_local(NonLocalUnmappedName),
    
    /// Load an object from a local variable
    load(LocalName),
    load_non_local(NonLocalUnmappedName),

    /// Special for functions: Attach the current least_ancestor
    attach_ancestors,

    /// Swap the top 2 objects on the stack
    swap,

    /// Pop the top of the stack
    pop,

    /// Create a shallow (reference) copy of the top of the stack
    dup,
    
    /// Call the method of the (n + 2)nd object on the stack, with arguments the n top objects on
    /// the stack
    call_method(u8),

    /// Read the (n + 1)th object on the stack as a function object, and call it with arguments the
    /// n top objects on the stack
    call_function(u8),

    /// Get an attribute of an object, reading the top as an attribute and the 2nd to top as the
    /// object
    get_attr,

    set_attr,

    to_string,

    // Perform operations on the top 2 elements of the stack, leaving the result
    add,
    sub,
    mul,
    div,
    mod_,
    not,
    neg,
    or,
    and,
    cmp_lt,
    cmp_gt,
    cmp_eq,
    cmp_neq,
    cmp_leq,
    cmp_geq,

    /// Take the 2nd object on the stack, and take the 1st object as an index
    index_get,

    index_set,
    
    /// Transform the object on the top of the stack into an interator object
    make_iter,

    /// Take the next element from the iterator on top of the stack. Jump if empty
    take_iter(i16),

    /// Take the top n elements of the stack and put them in a list
    mklist(u16),

    mktuple(u16),

    push_int(i32),

    push_float(f32),
    
    /// Push a constant referred to by a global constant descriptor
    push_const(GlobalConstantDescriptor),

    /// Push a constant referred to by a global constant descriptor, and make a deep clone
    push_const_clone(GlobalConstantDescriptor),
    
    /// Push a constant built in object / default (see: standard)
    push_global_default(GlobalConstantDescriptor),

    /// Jump to a relative offset in the instructions
    jmp(i16),
    
    /// Jump if the top of the stack is truthy
    cond_jmp(i16),

    /// Return the top of the stack from the current function
    ret,
    
    /// Launch as a subprocess the string on the top of the stack
    sh,

    /// Attach a debug reference to the next instruction in case it errors
    debug(DebugSpanDescriptor),
    
    /// Attach a weak debug reference to the next instruction in case it errors without a message
    weak_debug(DebugSpanDescriptor),
}

#[derive(Debug)]
pub struct GlobalContext {
    pub constant_descriptors: HashMap<GlobalConstantDescriptor, ObjectRef>,
    pub debug_descriptors: HashMap<DebugSpanDescriptor, Span>,
}

#[derive(Debug)]
pub struct Frame<'code> {
    id: FrameId,
    context_id: ContextId,
    global_context: Arc<GlobalContext>,
    code: &'code [Op],
    curr_instruction: usize,
    least_ancestors: HashMap<ContextId, FrameId>,
    pub stack: Vec<ObjectRef>,
    pub locals: &'code mut HashMap<NonLocalName, ObjectRef>,
}

macro_rules! try_debug {
    ($this: expr, $debug_symb: expr, $weak_debug_symb: expr, $expr: expr) => {
        $expr.map_err(|e| match $debug_symb {
            None => RuntimeError::from(e),
            Some(debug_symb) => {
                let span = $this.global_context.debug_descriptors.get(&debug_symb);
                if let Some(span) = span {
                    RuntimeError::from(e).attach_span(*span)
                } else {
                    RuntimeError::from(e)
                }
            },
        }).map_err(|e| match $weak_debug_symb {
            None => RuntimeError::from(e),
            Some(weak_debug_symb) => {
                let span = $this.global_context.debug_descriptors.get(&weak_debug_symb);
                if let Some(span) = span {
                    RuntimeError::from(e).weak_attach_span(*span)
                } else {
                    RuntimeError::from(e)
                }
            }
        })?;
    };
}

impl<'code> Frame<'code> {
    pub fn new(code: &'code [Op], locals: &'code mut HashMap<NonLocalName, ObjectRef>, globals: Arc<GlobalContext>, least_ancestors: HashMap<ContextId, FrameId>, context_id: ContextId) -> Self {
        Frame {
            id: gen_frame_id(),
            context_id,
            global_context: globals,
            code,
            curr_instruction: 0,
            locals,
            stack: vec![],
            least_ancestors,
        }
    }

    pub fn run(&mut self) -> Result<ObjectRef> {
        let mut stale_debug_symb = false;
        let mut stale_weak_debug_symb = false;
        let mut ds: Option<DebugSpanDescriptor> = None;
        let mut dsw: Option<DebugSpanDescriptor> = None;
                    
        self.least_ancestors.insert(self.context_id, self.id);

        loop {
            if !stale_debug_symb {
                stale_debug_symb = true;
            }
            if !stale_weak_debug_symb {
                stale_weak_debug_symb = true;
            }
            let instr = self.code.get(self.curr_instruction);
            if let None = instr {
                return Ok(VoidObject::new());
            }
            let instr = instr.unwrap();
            match instr {
                Op::nop => {},
                Op::store(local_name) => {
                    let res = self.stack.pop();
                    if let Some(val) = res {
                        self.locals.insert((self.id, *local_name), val);
                    } else {
                        return Err(RuntimeError::internal_error("Stored an empty stack!".to_string()));
                    }
                },
                Op::store_non_local(nl_name) => {
                    let res = self.stack.pop();
                    if let Some(val) = res {
                        self.locals.insert((*self.least_ancestors.get(&nl_name.0).unwrap(), nl_name.1), val);
                    } else {
                        return Err(RuntimeError::internal_error("Stored an empty stack!".to_string()));
                    }
                },
                Op::load(local_name) => {
                    let local = self.locals.get(&(self.id, *local_name));
                    if let Some(val) = local {
                        self.stack.push(Arc::clone(val));
                    } else {
                        return Err(RuntimeError::internal_error("Loaded a local that doesn't exist!".to_string()));
                    }
                },
                Op::load_non_local(nl_name) => {
                    let nl = self.locals.get(&(*self.least_ancestors.get(&nl_name.0).unwrap(), nl_name.1));
                    if let Some(val) = nl {
                        self.stack.push(Arc::clone(val));
                    } else {
                        return Err(RuntimeError::internal_error("Loaded a local that doesn't exist!".to_string()));
                    }
                },
                Op::attach_ancestors => {
                    let top = self.stack.pop();
                    if let Some(top) = top {
                        let top_any = top.as_any();
                        if let Some(f) = top_any.downcast_ref::<Function>() {
                            let mut la = f.least_ancestors.lock().unwrap();
                            assert!(la.is_none());
                            *la = Some(self.least_ancestors.clone());
                        } else {
                            return Err(RuntimeError::internal_error("Tried to attach ancestors to non-function".to_string()));
                        }
                        self.stack.push(top);
                    } else {
                        return Err(RuntimeError::internal_error("Tried to attach ancestors to nothing".to_string()));
                    }
                },
                Op::swap => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Stack is too small to swap!".to_string()));
                    }
                    let top = self.stack.pop().unwrap();
                    let snd = self.stack.pop().unwrap();
                    self.stack.push(top);
                    self.stack.push(snd);
                },
                Op::pop => {
                    let res = self.stack.pop();
                    if res.is_none() {
                        return Err(RuntimeError::internal_error("Popped an empty stack!".to_string()));
                    }
                },
                Op::dup => {
                    let dup = self.stack.last().map(|val| Arc::clone(val));
                    if let Some(val) = dup {
                        self.stack.push(val);
                    } else {
                        return Err(RuntimeError::internal_error("Dupped an empty stack!".to_string()));
                    }
                },
                Op::call_method(nargs) => {
                    let nargs = *nargs as usize;
                    if self.stack.len() < nargs + 2 {
                        return Err(RuntimeError::internal_error("Called method on too small a stack!".to_string()));
                    }
                    let args: Vec<ObjectRef> = self.stack.drain((self.stack.len() - nargs)..).collect();
                    let name = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    let name = name.as_any();
                    if let Some(method_name) = name.downcast_ref::<StringObject>() {
                        let res = try_debug!(self, ds, dsw, obj.call_method(method_name.val.as_ref(), &args));
                        self.stack.push(res);
                    } else {
                        return Err(RuntimeError::internal_error("Method name not a string!".to_string()));
                    }
                },
                Op::call_function(nargs) => {
                    let nargs = *nargs as usize;
                    if self.stack.len() < nargs + 1 {
                        return Err(RuntimeError::internal_error("Called function object on too small a stack!".to_string()));
                    }
                    let args: Vec<ObjectRef> = self.stack.drain((self.stack.len() - nargs)..).collect();
                    let func = self.stack.pop().unwrap();
                    let res = try_debug!(self, ds, dsw, func.call(&args, &mut self.locals));
                    self.stack.push(res);
                },
                Op::get_attr => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Too small a stack to perform get_attr!".to_string()));
                    }
                    let attr = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    let attr = attr.as_any();
                    if let Some(attr_name) = attr.downcast_ref::<StringObject>() {
                        let res = try_debug!(self, ds, dsw, obj.get_attr(RustClone::clone(&attr_name.val)));
                        self.stack.push(res);
                    } else {
                        return Err(RuntimeError::internal_error("Attribute name not a string!".to_string()));
                    }
                },
                Op::set_attr => {
                    if self.stack.len() < 3 {
                        return Err(RuntimeError::internal_error("Too small a stack to perform set_attr!".to_string()));
                    }
                    let toset = self.stack.pop().unwrap();
                    let attr = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    let attr = attr.as_any();
                    if let Some(attr_name) = attr.downcast_ref::<StringObject>() {
                        try_debug!(self, ds, dsw, obj.set_attr(RustClone::clone(&attr_name.val), toset));
                    } else {
                        return Err(RuntimeError::internal_error("Attribute name not a string!".to_string()));
                    }
                },
                Op::to_string => {
                    let obj = self.stack.pop();
                    if let Some(obj) = obj {
                        self.stack.push(StringObject::new(try_debug!(self, ds, dsw, obj.to_string())));
                    } else {
                        return Err(RuntimeError::internal_error("to_string called on an empty stack!".to_string()));
                    }
                },
                Op::add => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to add less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::add(b, a)))
                },
                Op::sub => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to sub less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::sub(b, a)))
                },
                Op::mul => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to mul less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::mul(b, a)))
                },
                Op::div => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to div less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::div(b, a)))
                },
                Op::mod_ => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to mod less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::mod_(b, a)))
                },
                Op::not => {
                    if self.stack.len() < 1 {
                        return Err(RuntimeError::internal_error("Tried to not nothing!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::not(a)));
                },
                Op::neg => {
                    if self.stack.len() < 1 {
                        return Err(RuntimeError::internal_error("Tried to negate nothing!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::negate(a)));
                },
                Op::or => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to div less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::or(b, a)))
                },
                Op::and => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to div less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::and(b, a)))
                },
                Op::cmp_lt => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::cmp_lt(b, a)))
                },
                Op::cmp_gt => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::cmp_gt(b, a)))
                },
                Op::cmp_eq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::cmp_eq(b, a)))
                },
                Op::cmp_neq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::cmp_neq(b, a)))
                },
                Op::cmp_leq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::cmp_leq(b, a)))
                },
                Op::cmp_geq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::cmp_geq(b, a)))
                },
                Op::index_get => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to index less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::index_get(b, a)))
                },
                Op::index_set => {
                    if self.stack.len() < 3 {
                        return Err(RuntimeError::internal_error("Tried to index less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let c = self.stack.pop().unwrap();
                    try_debug!(self, ds, dsw, builtins::index_set(c, b, a))
                },
                Op::make_iter => {
                    let val = self.stack.pop();
                    if let Some(val) = val {
                        self.stack.push(try_debug!(self, ds, dsw, val.make_iter()));
                    } else {
                        return Err(RuntimeError::internal_error("Tried to call make_iter on nothing!".to_string()));
                    }
                },
                Op::take_iter(offset) => {
                    let val = self.stack.pop();
                    if let Some(val) = val {
                        let val = try_debug!(self, ds, dsw, val.take_iter());
                        if let Some(val) = val {
                            self.stack.push(val);
                        } else {
                            // Jump
                            if *offset > 0 {
                                let offset: usize = *offset as u16 as usize;
                                self.curr_instruction += offset;
                            } else {
                                let offset: usize = (-offset) as u16 as usize;
                                self.curr_instruction -= offset;
                            }
                            continue;
                        }
                    } else {
                        return Err(RuntimeError::internal_error("Tried to call make_iter on nothing!".to_string()));
                    }
                },
                Op::mklist(len) => {
                    let len = *len as usize;
                    let objs: Vec<ObjectRef> = self.stack.drain((self.stack.len() - len)..).collect();
                    self.stack.push(Arc::new(List { contents: Mutex::new(objs) } ));
                },
                Op::mktuple(len) => {
                    let len = *len as usize;
                    let objs: Vec<ObjectRef> = self.stack.drain((self.stack.len() - len)..).collect();
                    self.stack.push(Arc::new(Tuple { contents: objs } ));
                },
                Op::push_int(int_val) => {
                    let obj = IntObject::new(*int_val as i64);
                    self.stack.push(obj);
                },
                Op::push_float(f_val) => {
                    let obj = FloatObject::new(*f_val as f64);
                    self.stack.push(obj);
                },
                Op::push_const(const_descr) => {
                    let obj = self.global_context.constant_descriptors.get(const_descr);
                    if let Some(obj) = obj {
                        self.stack.push(Arc::clone(obj));
                    } else {
                        return Err(RuntimeError::internal_error("Reference to constant that doesn't exist!".to_string()));
                    }
                },
                Op::push_const_clone(const_descr) => {
                    let obj = self.global_context.constant_descriptors.get(const_descr);
                    if let Some(obj) = obj {
                        self.stack.push(obj.marsh_clone()?);
                    } else {
                        return Err(RuntimeError::internal_error("Reference to constant that doesn't exist!".to_string()));
                    }
                },
                Op::push_global_default(const_descr) => {
                    let obj = Default_Namespace.get(const_descr);
                    if let Some(obj) = obj {
                        self.stack.push(Arc::clone(obj));
                    } else {
                        return Err(RuntimeError::internal_error("Reference to a global default that doesn't exist!".to_string()));
                    }
                },
                Op::jmp(offset) => {
                    if *offset > 0 {
                        let offset: usize = *offset as u16 as usize;
                        self.curr_instruction += offset;
                    } else {
                        let offset: usize = (-offset) as u16 as usize;
                        self.curr_instruction -= offset;
                    }
                    continue;
                },
                Op::cond_jmp(offset) => {
                    let obj = self.stack.pop();
                    if let Some(obj) = obj {
                        if obj.truthy() {
                            if *offset > 0 {
                                let offset: usize = *offset as u16 as usize;
                                self.curr_instruction += offset;
                            } else {
                                let offset: usize = (-offset) as u16 as usize;
                                self.curr_instruction -= offset;
                            }
                            continue;
                        }
                    } else {
                        return Err(RuntimeError::internal_error("cond_jmp on an empty stack!".to_string()));
                    }
                },
                Op::ret => {
                    let res = self.stack.pop();
                    if let Some(val) = res {
                        return Ok(val);
                    } else {
                        return Err(RuntimeError::internal_error("Returned an empty stack!".to_string()));
                    }
                },
                Op::sh => {
                    let top = self.stack.pop();
                    if let Some(top) = top {
                        let top = top.as_any();
                        if let Some(top) = top.downcast_ref::<StringObject>() {
                            let arg = top.val.clone();
                            let mut parts = arg.trim().split(' '); // TODO Temporary, add splitting on pipes etc.
                            let mut command = Command::new(parts.next().unwrap_or(""));
                            command.args(parts);
                            let process = command.spawn();
                            if let Ok(mut child) = process {
                                try_debug!(self, ds, dsw, child.wait());
                            } else {
                                let mut err = RuntimeError::child_process_error("Child process failed to start".to_string());
                                if let Some(ds) = ds {
                                    return Err(err.attach_span(*self.global_context.debug_descriptors.get(&ds).unwrap()));
                                }
                                return Err(err);
                            }
                        } else {
                            return Err(RuntimeError::internal_error("Tried to call sh on a non-string!".to_string()));
                        }
                    } else {
                        return Err(RuntimeError::internal_error("Tried to call sh on an empty stack!".to_string()));
                    }
                },
                Op::debug(symb) => {
                    ds = Some(*symb);
                    stale_debug_symb = false;
                },
                Op::weak_debug(symb) => {
                    dsw = Some(*symb);
                    stale_weak_debug_symb = false;
                },
            }
            
            if stale_debug_symb {
                ds = None;
            }
            if stale_weak_debug_symb {
                dsw = None;
            }
            self.curr_instruction += 1;
        }
    }
}
