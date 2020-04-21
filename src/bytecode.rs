
use std::collections::HashMap;

pub type LocalName = u16;
pub type GlobalConstantDescriptor = u16;

/// An operation in the rush virtual machine
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
    copy,
    
    /// Call the method of the (n + 1)th object on the stack, with arguments the n top objects on
    /// the stack
    call_method(u8),

    /// Read the (n + 1)th object on the stack as a function object, and call it with arguments the
    /// n top objects on the stack
    call_function(u8),

    /// Get an attribute of an object, reading the top as an attribute and the 2nd to top as the
    /// object
    get_attr,

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
    cmp_leq,
    cmp_geq,
    
    /// Push a constant referred to by a global constant descriptor
    push_const(GlobalConstantDescriptor),

    /// Jump to a relative offset in the instructions
    jmp(u8),
    
    /// Jump if the top of the stack is truthy
    cond_jmp(u8),

    /// Return the top of the stack from the current function
    ret,
}

pub struct Frame<'parent, 'code: 'parent> {
    parent: Option<Box<Frame<'parent>>>,
    code: &'code [Op],
    curr_instruction: usize,
    locals: Vec<ObjectRef>,
}
