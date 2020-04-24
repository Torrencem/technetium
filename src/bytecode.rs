
use std::collections::HashMap;

use crate::core::*;

use std::sync::Arc;

use std::clone::Clone as RustClone;

use crate::builtins;
use crate::standard::Default_Namespace;

use std::fmt;

use codespan::Span;

pub type LocalName = u16;
pub type GlobalConstantDescriptor = u16;
pub type DebugSpanDescriptor = u16;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// An operation in the marsh virtual machine
pub enum Op {
    /// Do nothing
    nop,
    
    /// Store the object on the top of the stack in a local variable
    store(LocalName),
    
    /// Load an object from a local variable
    load(LocalName),

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
    index,
    
    /// Transform the object on the top of the stack into an interator object
    make_iter,

    /// Take the next element from the iterator on top of the stack. Jump if empty
    take_iter(i16),

    /// Take the top n elements of the stack and put them in a list
    mklist(u16),

    mktuple(u16),
    
    /// Push a constant referred to by a global constant descriptor
    push_const(GlobalConstantDescriptor),
    
    /// Push a constant built in object / default (see: standard)
    push_global_default(GlobalConstantDescriptor),

    /// Jump to a relative offset in the instructions
    jmp(i16),
    
    /// Jump if the top of the stack is truthy
    cond_jmp(i16),

    /// Return the top of the stack from the current function
    ret,

    /// Attach a debug reference to the next instruction in case it errors
    debug(DebugSpanDescriptor),
    
    /// Attach a weak debug reference to the next instruction in case it errors without a message
    weak_debug(DebugSpanDescriptor),

}

pub struct GlobalContext {
    pub constant_descriptors: HashMap<GlobalConstantDescriptor, ObjectRef>,
    pub debug_descriptors: HashMap<DebugSpanDescriptor, Span>,
}

pub struct Frame<'code> {
    global_context: Arc<GlobalContext>,
    code: &'code [Op],
    curr_instruction: usize,
    pub stack: Vec<ObjectRef>,
    pub locals: &'code mut HashMap<LocalName, ObjectRef>,
}

macro_rules! try_debug {
    ($this: expr, $debug_symb: expr, $weak_debug_symb: expr, $expr: expr) => {
        $expr.map_err(|e| match $debug_symb {
            None => e,
            Some(debug_symb) => {
                let span = $this.global_context.debug_descriptors.get(&debug_symb);
                if let Some(span) = span {
                    e.attach_span(*span)
                } else {
                    e
                }
            },
        }).map_err(|e| match $weak_debug_symb {
            None => e,
            Some(weak_debug_symb) => {
                let span = $this.global_context.debug_descriptors.get(&weak_debug_symb);
                if let Some(span) = span {
                    e.weak_attach_span(*span)
                } else {
                    e
                }
            }
        })?;
    };
}

impl<'code> Frame<'code> {
    pub fn new(code: &'code [Op], locals: &'code mut HashMap<LocalName, ObjectRef>, globals: Arc<GlobalContext>) -> Self {
        Frame {
            global_context: globals,
            code,
            curr_instruction: 0,
            locals,
            stack: vec![],
        }
    }

    pub fn run(&mut self) -> Result<ObjectRef> {
        let mut stale_debug_symb = false;
        let mut stale_weak_debug_symb = false;
        let mut ds: Option<DebugSpanDescriptor> = None;
        let mut dsw: Option<DebugSpanDescriptor> = None;
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
                        self.locals.insert(*local_name, val);
                    } else {
                        return Err(RuntimeError::internal_error("Stored an empty stack!".to_string()));
                    }
                },
                Op::load(local_name) => {
                    let local = self.locals.get(local_name);
                    if let Some(val) = local {
                        self.stack.push(Arc::clone(val));
                    } else {
                        return Err(RuntimeError::internal_error("Loaded a local that doesn't exist!".to_string()));
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
                    if let Some(method_name) = name.downcast_ref::<String>() {
                        let res = try_debug!(self, ds, dsw, obj.call_method(method_name, &args));
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
                    if let Some(attr_name) = attr.downcast_ref::<String>() {
                        let res = try_debug!(self, ds, dsw, obj.get_attr(RustClone::clone(attr_name)));
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
                    if let Some(attr_name) = attr.downcast_ref::<String>() {
                        try_debug!(self, ds, dsw, obj.set_attr(RustClone::clone(attr_name), toset));
                    } else {
                        return Err(RuntimeError::internal_error("Attribute name not a string!".to_string()));
                    }
                },
                Op::to_string => {
                    let obj = self.stack.pop();
                    if let Some(obj) = obj {
                        self.stack.push(Arc::new(try_debug!(self, ds, dsw, obj.to_string())));
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
                Op::index => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to index less than 2 things!".to_string()));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::index(b, a)))
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
                    self.stack.push(Arc::new(List { contents: objs } ));
                },
                Op::mktuple(len) => {
                    let len = *len as usize;
                    let objs: Vec<ObjectRef> = self.stack.drain((self.stack.len() - len)..).collect();
                    self.stack.push(Arc::new(Tuple { contents: objs } ));
                },
                Op::push_const(const_descr) => {
                    let obj = self.global_context.constant_descriptors.get(const_descr);
                    if let Some(obj) = obj {
                        self.stack.push(Arc::clone(obj));
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
