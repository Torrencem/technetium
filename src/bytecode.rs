
use std::collections::HashMap;

use crate::core::*;

use std::sync::Arc;

use std::clone::Clone as RustClone;

use crate::builtins;

use std::fmt;

pub type LocalName = u16;
pub type GlobalConstantDescriptor = u16;

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

    /// Jump to a relative offset in the instructions
    jmp(i16),
    
    /// Jump if the top of the stack is truthy
    cond_jmp(i16),

    /// Return the top of the stack from the current function
    ret,
}

pub struct GlobalContext {
    pub constant_descriptors: HashMap<GlobalConstantDescriptor, ObjectRef>,
}

pub struct Frame<'code> {
    global_context: Arc<GlobalContext>,
    code: &'code [Op],
    curr_instruction: usize,
    pub locals: HashMap<LocalName, ObjectRef>,
}

impl<'code> Frame<'code> {
    pub fn new(code: &'code [Op], globals: Arc<GlobalContext>) -> Self {
        Frame {
            global_context: globals,
            code: code,
            curr_instruction: 0,
            locals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<ObjectRef> {
        let mut stack: Vec<ObjectRef> = vec![];
        loop {
            let instr = self.code.get(self.curr_instruction);
            if let None = instr {
                return Err(RuntimeError::internal_error("Ran off the end of the code!".to_string()));
            }
            let instr = instr.unwrap();
            match instr {
                Op::nop => {},
                Op::store(local_name) => {
                    let res = stack.pop();
                    if let Some(val) = res {
                        self.locals.insert(*local_name, val);
                    } else {
                        return Err(RuntimeError::internal_error("Stored an empty stack!".to_string()));
                    }
                },
                Op::load(local_name) => {
                    let local = self.locals.get(local_name);
                    if let Some(val) = local {
                        stack.push(Arc::clone(val));
                    } else {
                        dbg!(&self.locals);
                        dbg!(&local_name);
                        return Err(RuntimeError::internal_error("Loaded a local that doesn't exist!".to_string()));
                    }
                },
                Op::swap => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Stack is too small to swap!".to_string()));
                    }
                    let top = stack.pop().unwrap();
                    let snd = stack.pop().unwrap();
                    stack.push(top);
                    stack.push(snd);
                },
                Op::pop => {
                    let res = stack.pop();
                    if res.is_none() {
                        return Err(RuntimeError::internal_error("Popped an empty stack!".to_string()));
                    }
                },
                Op::dup => {
                    let dup = stack.last().map(|val| Arc::clone(val));
                    if let Some(val) = dup {
                        stack.push(val);
                    } else {
                        return Err(RuntimeError::internal_error("Dupped an empty stack!".to_string()));
                    }
                },
                Op::call_method(nargs) => {
                    let nargs = *nargs as usize;
                    if stack.len() < nargs + 2 {
                        return Err(RuntimeError::internal_error("Called method on too small a stack!".to_string()));
                    }
                    let args: Vec<ObjectRef> = stack.drain((stack.len() - nargs)..).collect();
                    let name = stack.pop().unwrap();
                    let obj = stack.pop().unwrap();
                    let name = name.as_any();
                    if let Some(method_name) = name.downcast_ref::<String>() {
                        let res = obj.call_method(method_name, &args)?;
                        stack.push(res);
                    } else {
                        return Err(RuntimeError::internal_error("Method name not a string!".to_string()));
                    }
                },
                Op::call_function(nargs) => {
                    let nargs = *nargs as usize;
                    if stack.len() < nargs + 1 {
                        return Err(RuntimeError::internal_error("Called function object on too small a stack!".to_string()));
                    }
                    let args: Vec<ObjectRef> = stack.drain((stack.len() - nargs)..).collect();
                    let func = stack.pop().unwrap();
                    let res = func.call(&args)?;
                    stack.push(res);
                },
                Op::get_attr => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Too small a stack to perform get_attr!".to_string()));
                    }
                    let attr = stack.pop().unwrap();
                    let obj = stack.pop().unwrap();
                    let attr = attr.as_any();
                    if let Some(attr_name) = attr.downcast_ref::<String>() {
                        let res = obj.get_attr(RustClone::clone(attr_name))?;
                        stack.push(res);
                    } else {
                        return Err(RuntimeError::internal_error("Attribute name not a string!".to_string()));
                    }
                },
                Op::set_attr => {
                    if stack.len() < 3 {
                        return Err(RuntimeError::internal_error("Too small a stack to perform set_attr!".to_string()));
                    }
                    let toset = stack.pop().unwrap();
                    let attr = stack.pop().unwrap();
                    let obj = stack.pop().unwrap();
                    let attr = attr.as_any();
                    if let Some(attr_name) = attr.downcast_ref::<String>() {
                        obj.set_attr(RustClone::clone(attr_name), toset)?;
                    } else {
                        return Err(RuntimeError::internal_error("Attribute name not a string!".to_string()));
                    }
                },
                Op::to_string => {
                    let obj = stack.pop();
                    if let Some(obj) = obj {
                        stack.push(Arc::new(obj.to_string()?));
                    } else {
                        return Err(RuntimeError::internal_error("to_string called on an empty stack!".to_string()));
                    }
                },
                Op::add => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to add less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::add(b, a)?)
                },
                Op::sub => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to sub less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::sub(b, a)?)
                },
                Op::mul => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to mul less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::mul(b, a)?)
                },
                Op::div => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to div less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::div(b, a)?)
                },
                Op::not => {
                    if stack.len() < 1 {
                        return Err(RuntimeError::internal_error("Tried to not nothing!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    stack.push(builtins::not(a)?);
                },
                Op::or => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to div less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::or(b, a)?)
                },
                Op::and => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to div less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::and(b, a)?)
                },
                Op::cmp_lt => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::cmp_lt(a, b)?)
                },
                Op::cmp_gt => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::cmp_gt(a, b)?)
                },
                Op::cmp_eq => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::cmp_eq(a, b)?)
                },
                Op::cmp_neq => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::cmp_neq(a, b)?)
                },
                Op::cmp_leq => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::cmp_leq(a, b)?)
                },
                Op::cmp_geq => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to compare less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::cmp_geq(a, b)?)
                },
                Op::index => {
                    if stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Tried to index less than 2 things!".to_string()));
                    }
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(builtins::index(b, a)?)
                },
                Op::make_iter => {
                    let val = stack.pop();
                    if let Some(val) = val {
                        stack.push(val.make_iter()?);
                    } else {
                        return Err(RuntimeError::internal_error("Tried to call make_iter on nothing!".to_string()));
                    }
                },
                Op::take_iter(offset) => {
                    let val = stack.pop();
                    if let Some(val) = val {
                        let val = val.take_iter()?;
                        if let Some(val) = val {
                            stack.push(val);
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
                    let objs: Vec<ObjectRef> = stack.drain((stack.len() - len)..).collect();
                    stack.push(Arc::new(List { contents: objs } ));
                },
                Op::mktuple(len) => {
                    let len = *len as usize;
                    let objs: Vec<ObjectRef> = stack.drain((stack.len() - len)..).collect();
                    stack.push(Arc::new(Tuple { contents: objs } ));
                },
                Op::push_const(const_descr) => {
                    let obj = self.global_context.constant_descriptors.get(const_descr);
                    if let Some(obj) = obj {
                        stack.push(Arc::clone(obj));
                    } else {
                        return Err(RuntimeError::internal_error("Reference to constant that doesn't exist!".to_string()));
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
                    let obj = stack.pop();
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
                    let res = stack.pop();
                    if let Some(val) = res {
                        return Ok(val);
                    } else {
                        return Err(RuntimeError::internal_error("Returned an empty stack!".to_string()));
                    }
                },
            }

            self.curr_instruction += 1;
        }
    }
}


