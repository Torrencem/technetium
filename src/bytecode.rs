use std::collections::HashMap;

use crate::core::*;
use crate::error::*;

use std::sync::{Arc, Mutex};

use std::clone::Clone as RustClone;
use std::io::{self, Write};
use std::process::{Command, Stdio};

use crate::builtins;
use crate::standard::Default_Namespace;
use crate::standard::conversion;

use std::fmt;

use std::any::TypeId;

use codespan::Span;

pub type Bytecode = Vec<Op>;
pub type CompileResult = std::result::Result<Bytecode, CompileError>;

/// An identifier unique to each frame of computation
pub type FrameId = u32;

pub struct FrameIdGen {
    last: FrameId,
}

lazy_static! {
    pub static ref FRAME_ID_GEN: Mutex<FrameIdGen> = { Mutex::new(FrameIdGen { last: 100 }) };
}

/// Generate a new unique FrameID. This uses a static counter behind
/// a mutex to guarantee a unique ID.
pub fn gen_frame_id() -> FrameId {
    let mut gen = FRAME_ID_GEN.lock().unwrap();
    let old = gen.last;
    gen.last += 1;
    old
}

/// Unique to each function in source code. Unlike FrameId, multiple
/// frames can share a ContextID if they came from the same function
/// of source code.
pub type ContextId = u16;

pub type LocalName = u16;
pub type NonLocalUnmappedName = (ContextId, u16);
pub type NonLocalName = (FrameId, u16);
pub type GlobalConstantDescriptor = (ContextId, u16);
pub type DebugSpanDescriptor = u16;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
/// An operation on the technetium virtual machine
pub enum Op {
    /// Do nothing
    nop,

    /// Store the object on the top of the stack in a local variable
    store(LocalName),

    /// Store the object on the top of the stack in a local variable
    /// in a different frame
    store_non_local(NonLocalUnmappedName),

    /// Load an object from a local variable
    load(LocalName),

    /// Load an object from a local variable in a different frame
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

    fmt_string(u8),

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

    index_get,

    index_set,

    make_slice,

    /// Transform the object on the top of the stack into an interator object
    make_iter,

    /// Take the next element from the iterator on top of the stack. Jump if empty
    take_iter(i16),

    /// Take the top n elements of the stack and put them in a list
    mklist(u16),

    mktuple(u16),

    push_int(i32),

    push_float(f32),

    push_bool(bool),

    /// Push a constant referred to by a global constant descriptor
    push_const(GlobalConstantDescriptor),

    /// Push a constant referred to by a global constant descriptor, and make a deep clone
    push_const_clone(GlobalConstantDescriptor),

    /// Push a constant built in object / default (see: standard)
    push_global_default(GlobalConstantDescriptor),

    push_void,

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
        $expr
            .map_err(|e| match $debug_symb {
                None => RuntimeError::from(e),
                Some(debug_symb) => {
                    let span = $this.global_context.debug_descriptors.get(&debug_symb);
                    if let Some(span) = span {
                        RuntimeError::from(e).attach_span(*span)
                    } else {
                        RuntimeError::from(e)
                    }
                }
            })
            .map_err(|e| match $weak_debug_symb {
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
    pub fn new(
        code: &'code [Op],
        locals: &'code mut HashMap<NonLocalName, ObjectRef>,
        globals: Arc<GlobalContext>,
        least_ancestors: HashMap<ContextId, FrameId>,
        context_id: ContextId,
    ) -> Self {
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

    pub fn run(&mut self) -> RuntimeResult<ObjectRef> {
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
                Op::nop => {}
                Op::store(local_name) => {
                    let res = self.stack.pop();
                    if let Some(val) = res {
                        self.locals.insert((self.id, *local_name), val);
                    } else {
                        return Err(RuntimeError::internal_error("Stored an empty stack!"));
                    }
                }
                Op::store_non_local(nl_name) => {
                    let res = self.stack.pop();
                    if let Some(val) = res {
                        self.locals.insert(
                            (*self.least_ancestors.get(&nl_name.0).unwrap(), nl_name.1),
                            val,
                        );
                    } else {
                        return Err(RuntimeError::internal_error("Stored an empty stack!"));
                    }
                }
                Op::load(local_name) => {
                    let local = self.locals.get(&(self.id, *local_name));
                    if let Some(val) = local {
                        self.stack.push(Arc::clone(val));
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Loaded a local that doesn't exist!",
                        ));
                    }
                }
                Op::load_non_local(nl_name) => {
                    let nl = self
                        .locals
                        .get(&(*self.least_ancestors.get(&nl_name.0).unwrap(), nl_name.1));
                    if let Some(val) = nl {
                        self.stack.push(Arc::clone(val));
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Loaded a local that doesn't exist!",
                        ));
                    }
                }
                Op::attach_ancestors => {
                    let top = self.stack.pop();
                    if let Some(top) = top {
                        let top_any = top.as_any();
                        if let Some(f) = top_any.downcast_ref::<Function>() {
                            let mut la = f.least_ancestors.lock().unwrap();
                            assert!(la.is_none());
                            *la = Some(self.least_ancestors.clone());
                        } else {
                            return Err(RuntimeError::internal_error(
                                "Tried to attach ancestors to non-function",
                            ));
                        }
                        self.stack.push(top);
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Tried to attach ancestors to nothing",
                        ));
                    }
                }
                Op::swap => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error("Stack is too small to swap!"));
                    }
                    let top = self.stack.pop().unwrap();
                    let snd = self.stack.pop().unwrap();
                    self.stack.push(top);
                    self.stack.push(snd);
                }
                Op::pop => {
                    let res = self.stack.pop();
                    if res.is_none() {
                        return Err(RuntimeError::internal_error("Popped an empty stack!"));
                    }
                }
                Op::dup => {
                    let dup = self.stack.last().map(|val| Arc::clone(val));
                    if let Some(val) = dup {
                        self.stack.push(val);
                    } else {
                        return Err(RuntimeError::internal_error("Dupped an empty stack!"));
                    }
                }
                Op::call_method(nargs) => {
                    let nargs = *nargs as usize;
                    if self.stack.len() < nargs + 2 {
                        return Err(RuntimeError::internal_error(
                            "Called method on too small a stack!",
                        ));
                    }
                    let args: Vec<ObjectRef> =
                        self.stack.drain((self.stack.len() - nargs)..).collect();
                    let name = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    let name = name.as_any();
                    if let Some(method_name) = name.downcast_ref::<StringObject>() {
                        let val = method_name.val.lock().unwrap();
                        let res = try_debug!(self, ds, dsw, obj.call_method(val.as_ref(), &args));
                        self.stack.push(res);
                    } else {
                        return Err(RuntimeError::internal_error("Method name not a string!"));
                    }
                }
                Op::call_function(nargs) => {
                    let nargs = *nargs as usize;
                    if self.stack.len() < nargs + 1 {
                        return Err(RuntimeError::internal_error(
                            "Called function object on too small a stack!",
                        ));
                    }
                    let args: Vec<ObjectRef> =
                        self.stack.drain((self.stack.len() - nargs)..).collect();
                    let func = self.stack.pop().unwrap();
                    let res = try_debug!(self, ds, dsw, func.call(&args, &mut self.locals));
                    self.stack.push(res);
                }
                Op::get_attr => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Too small a stack to perform get_attr!",
                        ));
                    }
                    let attr = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    let attr = attr.as_any();
                    if let Some(attr_name) = attr.downcast_ref::<StringObject>() {
                        let val = attr_name.val.lock().unwrap();
                        let res = try_debug!(self, ds, dsw, obj.get_attr(val.clone()));
                        self.stack.push(res);
                    } else {
                        return Err(RuntimeError::internal_error("Attribute name not a string!"));
                    }
                }
                Op::set_attr => {
                    if self.stack.len() < 3 {
                        return Err(RuntimeError::internal_error(
                            "Too small a stack to perform set_attr!",
                        ));
                    }
                    let toset = self.stack.pop().unwrap();
                    let attr = self.stack.pop().unwrap();
                    let obj = self.stack.pop().unwrap();
                    let attr = attr.as_any();
                    if let Some(attr_name) = attr.downcast_ref::<StringObject>() {
                        let val = attr_name.val.lock().unwrap();
                        try_debug!(self, ds, dsw, obj.set_attr(val.clone(), toset));
                    } else {
                        return Err(RuntimeError::internal_error("Attribute name not a string!"));
                    }
                }
                Op::to_string => {
                    let obj = self.stack.pop();
                    if let Some(obj) = obj {
                        self.stack.push(StringObject::new(try_debug!(
                            self,
                            ds,
                            dsw,
                            obj.to_string()
                        )));
                    } else {
                        return Err(RuntimeError::internal_error(
                            "to_string called on an empty stack!",
                        ));
                    }
                }
                Op::fmt_string(num_args) => {
                    let len = *num_args as usize;
                    if self.stack.len() < len + 1 {
                        return Err(RuntimeError::internal_error(
                            "Tried to format a string with an incorrect number of args!",
                        ));
                    }
                    let mut objs: Vec<ObjectRef> =
                        self.stack.drain((self.stack.len() - len)..).collect();
                    let subs = self.stack.pop();
                    if let Some(subs) = subs {
                        if let Some(string) = subs.as_any().downcast_ref::<StringObject>() {
                            let mut result_string = String::new();
                            let val = string.val.lock().unwrap();
                            let mut chars = val.chars().peekable();
                            loop {
                                match chars.next() {
                                    Some('\\') => {
                                        if chars.peek() == Some(&'{') {
                                            chars.next();
                                            result_string.push('{');
                                        } else {
                                            result_string.push('\\');
                                        }
                                    }
                                    Some('{') => {
                                        let obj = objs.pop().unwrap();
                                        result_string.push_str(obj.to_string()?.as_ref());
                                    }
                                    Some(x) => result_string.push(x),
                                    None => break,
                                }
                            }

                            self.stack.push(StringObject::new(result_string));
                        } else {
                            return Err(RuntimeError::internal_error(
                                "Tried to format a non-string!",
                            ));
                        }
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Tried to format an empty stack!",
                        ));
                    }
                }
                Op::add => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to add less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::add(b, a)))
                }
                Op::sub => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to sub less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::sub(b, a)))
                }
                Op::mul => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to mul less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::mul(b, a)))
                }
                Op::div => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to div less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::div(b, a)))
                }
                Op::mod_ => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to mod less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::mod_(b, a)))
                }
                Op::not => {
                    if self.stack.len() < 1 {
                        return Err(RuntimeError::internal_error("Tried to not nothing!"));
                    }
                    let a = self.stack.pop().unwrap();
                    self.stack.push(try_debug!(self, ds, dsw, builtins::not(a)));
                }
                Op::neg => {
                    if self.stack.len() < 1 {
                        return Err(RuntimeError::internal_error("Tried to negate nothing!"));
                    }
                    let a = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::negate(a)));
                }
                Op::or => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to div less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::or(b, a)))
                }
                Op::and => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to div less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::and(b, a)))
                }
                Op::cmp_lt => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to compare less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::cmp_lt(b, a)))
                }
                Op::cmp_gt => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to compare less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::cmp_gt(b, a)))
                }
                Op::cmp_eq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to compare less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::cmp_eq(b, a)))
                }
                Op::cmp_neq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to compare less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::cmp_neq(b, a)))
                }
                Op::cmp_leq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to compare less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::cmp_leq(b, a)))
                }
                Op::cmp_geq => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to compare less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::cmp_geq(b, a)))
                }
                Op::index_get => {
                    if self.stack.len() < 2 {
                        return Err(RuntimeError::internal_error(
                            "Tried to index less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack
                        .push(try_debug!(self, ds, dsw, builtins::index_get(b, a)))
                }
                Op::index_set => {
                    if self.stack.len() < 3 {
                        return Err(RuntimeError::internal_error(
                            "Tried to index less than 2 things!",
                        ));
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let c = self.stack.pop().unwrap();
                    try_debug!(self, ds, dsw, builtins::index_set(c, b, a))
                }
                Op::make_slice => {
                    if self.stack.len() < 4 {
                        return Err(RuntimeError::internal_error(
                                "Tried to make a slice with too few arguments"
                        ));
                    }
                    let step = self.stack.pop().unwrap();
                    let step = if step.as_any().type_id() == TypeId::of::<VoidObject>() {
                        1
                    } else {
                        conversion::to_int(step)?
                    };
                    let stop = self.stack.pop().unwrap();
                    let stop = if stop.as_any().type_id() == TypeId::of::<VoidObject>() {
                        None
                    } else {
                        Some(conversion::to_int(stop)?)
                    };
                    let start = self.stack.pop().unwrap();
                    let start = if start.as_any().type_id() == TypeId::of::<VoidObject>() {
                        0
                    } else {
                        conversion::to_int(start)?
                    };
                    let parent = self.stack.pop().unwrap();
                    let slice = Slice {
                            parent,
                            start,
                            stop,
                            step,
                    };
                    self.stack.push(Arc::new(slice));
                }
                Op::make_iter => {
                    let val = self.stack.pop();
                    if let Some(val) = val {
                        self.stack.push(try_debug!(self, ds, dsw, val.make_iter()));
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Tried to call make_iter on nothing!",
                        ));
                    }
                }
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
                        return Err(RuntimeError::internal_error(
                            "Tried to call make_iter on nothing!",
                        ));
                    }
                }
                Op::mklist(len) => {
                    let len = *len as usize;
                    let objs: Vec<ObjectRef> =
                        self.stack.drain((self.stack.len() - len)..).collect();
                    self.stack.push(Arc::new(List {
                        contents: Mutex::new(objs),
                    }));
                }
                Op::mktuple(len) => {
                    let len = *len as usize;
                    let objs: Vec<ObjectRef> =
                        self.stack.drain((self.stack.len() - len)..).collect();
                    self.stack.push(Arc::new(Tuple { contents: objs }));
                }
                Op::push_int(int_val) => {
                    let obj = IntObject::new(*int_val as i64);
                    self.stack.push(obj);
                }
                Op::push_float(f_val) => {
                    let obj = FloatObject::new(*f_val as f64);
                    self.stack.push(obj);
                }
                Op::push_bool(b_val) => {
                    let obj = BoolObject::new(*b_val);
                    self.stack.push(obj);
                }
                Op::push_const(const_descr) => {
                    let obj = self.global_context.constant_descriptors.get(const_descr);
                    if let Some(obj) = obj {
                        self.stack.push(Arc::clone(obj));
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Reference to constant that doesn't exist!",
                        ));
                    }
                }
                Op::push_const_clone(const_descr) => {
                    let obj = self.global_context.constant_descriptors.get(const_descr);
                    if let Some(obj) = obj {
                        self.stack.push(obj.technetium_clone()?);
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Reference to constant that doesn't exist!",
                        ));
                    }
                }
                Op::push_global_default(const_descr) => {
                    let obj = Default_Namespace.get(const_descr);
                    if let Some(obj) = obj {
                        self.stack.push(Arc::clone(obj));
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Reference to a global default that doesn't exist!",
                        ));
                    }
                }
                Op::push_void => {
                    self.stack.push(VoidObject::new())
                }
                Op::jmp(offset) => {
                    if *offset > 0 {
                        let offset: usize = *offset as u16 as usize;
                        self.curr_instruction += offset;
                    } else {
                        let offset: usize = (-offset) as u16 as usize;
                        self.curr_instruction -= offset;
                    }
                    continue;
                }
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
                        return Err(RuntimeError::internal_error("cond_jmp on an empty stack!"));
                    }
                }
                Op::ret => {
                    let res = self.stack.pop();
                    if let Some(val) = res {
                        return Ok(val);
                    } else {
                        return Err(RuntimeError::internal_error("Returned an empty stack!"));
                    }
                }
                Op::sh => {
                    let top = self.stack.pop();
                    if let Some(top) = top {
                        let top = top.as_any();
                        if let Some(top) = top.downcast_ref::<StringObject>() {
                            let arg = top.val.lock().unwrap().clone();
                            let mut command = Command::new("sh");
                            let process = command.stdin(Stdio::piped()).spawn();
                            if let Ok(mut child) = process {
                                child.stdin.as_mut().unwrap().write_all(arg.as_bytes());
                                let exit_code = try_debug!(self, ds, dsw, child.wait());
                                if !exit_code.success() {
                                    let mut err = RuntimeError::child_process_error(format!(
                                        "Child process returned {}",
                                        exit_code
                                    ));
                                    if let Some(ds) = ds {
                                        return Err(err.attach_span(
                                            *self
                                                .global_context
                                                .debug_descriptors
                                                .get(&ds)
                                                .unwrap(),
                                        ));
                                    }
                                    return Err(err);
                                }
                            } else {
                                let mut err = RuntimeError::child_process_error(
                                    "Child process failed to start",
                                );
                                if let Some(ds) = ds {
                                    return Err(err.attach_span(
                                        *self.global_context.debug_descriptors.get(&ds).unwrap(),
                                    ));
                                }
                                return Err(err);
                            }
                        } else {
                            return Err(RuntimeError::internal_error(
                                "Tried to call sh on a non-string!",
                            ));
                        }
                    } else {
                        return Err(RuntimeError::internal_error(
                            "Tried to call sh on an empty stack!",
                        ));
                    }
                }
                Op::debug(symb) => {
                    ds = Some(*symb);
                    stale_debug_symb = false;
                }
                Op::weak_debug(symb) => {
                    dsw = Some(*symb);
                    stale_weak_debug_symb = false;
                }
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
