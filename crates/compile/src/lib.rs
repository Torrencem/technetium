
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub script);

pub mod error;
use error::*;
pub mod ast;
use ast::*;

use runtime::bytecode::*;
use runtime::*;
use runtime::memory::*;
use runtime::standard::STANDARD_CONTEXT_ID;
use runtime::standard::get_default_namespace_descriptors;

use codespan::FileId;
use codespan::Span;
use std::clone::Clone as RustClone;
use std::collections::HashMap;
use std::i32;
use parking_lot::RwLock;
use std::rc::Rc;

pub type Bytecode = Vec<Op>;
pub type CompileResult = std::result::Result<Bytecode, CompileError>;

/// Determine if a f64 is exactly representable as a f32
fn is_exact_float(val: f64) -> bool {
    ((val as f32) as f64) == val
}

pub struct CompileContext {
    file_id: FileId,
    context_id: ContextId,
    gcd_last: u16,
    pub constant_descriptors: HashMap<GlobalConstantDescriptor, ObjectRef>,
    dcd_last: DebugSpanDescriptor,
    pub debug_symbol_descriptors: HashMap<DebugSpanDescriptor, DebugSymbol>,
}

impl CompileContext {
    pub fn new(context_id: ContextId, file_id: FileId) -> CompileContext {
        CompileContext {
            file_id,
            context_id,
            gcd_last: 0,
            constant_descriptors: HashMap::new(),
            dcd_last: 0,
            debug_symbol_descriptors: HashMap::new(),
        }
    }

    pub fn gcd_gen(&mut self) -> GlobalConstantDescriptor {
        let old = self.gcd_last;
        self.gcd_last += 1;
        (self.context_id, old)
    }

    pub fn dsd_gen(&mut self) -> DebugSpanDescriptor {
        let old = self.dcd_last;
        self.dcd_last += 1;
        old
    }
}

fn builtin_functions() -> HashMap<String, Op> {
    let mut res = HashMap::new();
    res.insert("<add>".to_string(), Op::add);
    res.insert("<sub>".to_string(), Op::sub);
    res.insert("<mul>".to_string(), Op::mul);
    res.insert("<div>".to_string(), Op::div);
    res.insert("<mod>".to_string(), Op::mod_);
    res.insert("<eq>".to_string(), Op::cmp_eq);
    res.insert("<neq>".to_string(), Op::cmp_neq);
    res.insert("<gt>".to_string(), Op::cmp_gt);
    res.insert("<lt>".to_string(), Op::cmp_lt);
    res.insert("<geq>".to_string(), Op::cmp_geq);
    res.insert("<leq>".to_string(), Op::cmp_leq);
    res.insert("<and>".to_string(), Op::and);
    res.insert("<or>".to_string(), Op::or);
    res.insert("<not>".to_string(), Op::not);
    res.insert("<neg>".to_string(), Op::neg);
    res.insert("<index>".to_string(), Op::index_get);
    res
}

pub struct CompileManager {
    pub context_stack: Vec<CompileContext>,
    local_index_last: LocalName,
    pub local_index: HashMap<(ContextId, String), LocalName>,
    context_id_last: ContextId,
    default_namespace_descriptors: HashMap<String, GlobalConstantDescriptor>,
    pub memory_manager: MemoryManager,
}

pub enum NameLookupResult {
    MyLocal(LocalName),
    ExternLocal(NonLocalUnmappedName),
    Global(GlobalConstantDescriptor),
    NotFound,
}

impl CompileManager {
    pub fn new(file_id: FileId) -> Self {
        let mut mem_manager = MemoryManager::new();
        let context_id = STANDARD_CONTEXT_ID + 1;
        mem_manager.register_context(context_id);
        CompileManager {
            context_stack: vec![CompileContext::new(STANDARD_CONTEXT_ID + 1, file_id)],
            local_index_last: 0,
            local_index: HashMap::new(),
            context_id_last: STANDARD_CONTEXT_ID + 2,
            default_namespace_descriptors: get_default_namespace_descriptors(),
            memory_manager: mem_manager,
        }
    }

    pub fn context_id_gen(&mut self) -> ContextId {
        let old = self.context_id_last;
        self.context_id_last += 1;
        old
    }

    pub fn create_debug_descriptor(&mut self, span: Span) -> DebugSpanDescriptor {
        let debug_descr = self.context().dsd_gen();
        let file_id = self.context().file_id;
        self.context()
            .debug_symbol_descriptors
            .insert(debug_descr, DebugSymbol::new(file_id, span));
        debug_descr
    }

    pub fn create_constant_descriptor(&mut self, obj: ObjectRef) -> GlobalConstantDescriptor {
        let const_descr = self.context().gcd_gen();
        self.context()
            .constant_descriptors
            .insert(const_descr, obj);
        const_descr
    }

    pub fn context(&mut self) -> &mut CompileContext {
        self.context_stack.last_mut().unwrap()
    }

    pub fn local_name_gen(&mut self) -> LocalName {
        let old = self.local_index_last;
        self.local_index_last += 1;
        old
    }

    pub fn name_lookup(&mut self, name: &String) -> NameLookupResult {
        let mut first = true;
        for context in self.context_stack.iter().rev() {
            if let Some(local_index) = self.local_index.get(&(context.context_id, name.clone())) {
                if first {
                    return NameLookupResult::MyLocal(*local_index);
                } else {
                    // This unwrap *could* panic, but:
                    // 1. it would be an internal_error
                    // 2. it could happen at runtime or compile time, which
                    //    would be difficult to handle correctly.
                    self.memory_manager.do_not_drop(context.context_id, *local_index).expect("Internal memory manager error on do_not_drop");
                    return NameLookupResult::ExternLocal((context.context_id, *local_index));
                }
            }
            if first {
                first = false;
            }
        }
        if let Some(global_index) = self.default_namespace_descriptors.get(name) {
            NameLookupResult::Global(*global_index)
        } else {
            NameLookupResult::NotFound
        }
    }

    pub fn compile_literal(&mut self, ast: &Literal) -> CompileResult {
        let descr = self.context().gcd_gen();
        let constant: ObjectRef = match ast {
            Literal::Integer(val, _) => {
                if *val < i32::MAX as i64 && *val > i32::MIN as i64 {
                    return Ok(vec![Op::push_int(*val as i32)]);
                }
                IntObject::new(*val)
            }
            Literal::Float(val, _) => {
                if is_exact_float(*val) {
                    return Ok(vec![Op::push_float(*val as f32)]);
                }
                FloatObject::new(*val)
            }
            Literal::Bool(val, _) => BoolObject::new(*val),
            Literal::Str(val, _) => StringObject::new(RustClone::clone(val)),
            Literal::Char(val, _) => CharObject::new(*val),
            Literal::FormatString(f) => {
                return self.compile_format_string(&f);
            }
        };
        self.context().constant_descriptors.insert(descr, constant);
        Ok(vec![Op::push_const_clone(descr)])
    }

    pub fn compile_list_literal(&mut self, ast: &ListLiteral) -> CompileResult {
        let mut res = vec![];
        for item in ast.values.iter() {
            res.append(&mut self.compile_expr(item)?);
        }
        res.push(Op::mklist(ast.values.len() as u16));
        Ok(res)
    }

    pub fn compile_tuple_literal(&mut self, ast: &TupleLiteral) -> CompileResult {
        let mut res = vec![];
        for item in ast.values.iter() {
            res.append(&mut self.compile_expr(item)?);
        }
        res.push(Op::mktuple(ast.values.len() as u16));
        Ok(res)
    }
    
    pub fn compile_set_literal(&mut self, ast: &SetLiteral) -> CompileResult {
        let mut res = vec![];
        for item in ast.values.iter() {
            res.append(&mut self.compile_expr(item)?);
        }
        // Put a debug symbol in case an unhashable object was used
        res.push(Op::debug(self.create_debug_descriptor(ast.span)));

        res.push(Op::mkset(ast.values.len() as u16));
        Ok(res)
    }

    pub fn compile_func_call(&mut self, ast: &FuncCall) -> CompileResult {
        let mut res = vec![];
        let builtins = builtin_functions();
        if let Some(op) = builtins.get(&ast.fname.name) {
            // Special case <and> and <or> for short-circuiting
            if &ast.fname.name == "<and>" {
                assert!(ast.arguments.len() == 2);
                res.append(&mut self.compile_expr(&ast.arguments[0])?);
                let mut short_arg_2 = self.compile_expr(&ast.arguments[1])?;
                res.push(Op::push_bool(false));
                res.push(Op::swap);
                res.push(Op::not);
                res.push(Op::cond_jmp(short_arg_2.len() as u16 as i16 + 2));
                res.push(Op::pop);
                res.append(&mut short_arg_2);
                return Ok(res);
            } else if &ast.fname.name == "<or>" {
                assert!(ast.arguments.len() == 2);
                res.append(&mut self.compile_expr(&ast.arguments[0])?);
                let mut short_arg_2 = self.compile_expr(&ast.arguments[1])?;
                res.push(Op::push_bool(true));
                res.push(Op::swap);
                res.push(Op::cond_jmp(short_arg_2.len() as u16 as i16 + 2));
                res.push(Op::pop);
                res.append(&mut short_arg_2);
                return Ok(res);
            }
            for arg in ast.arguments.iter() {
                res.append(&mut self.compile_expr(arg)?);
            }
            res.push(*op);
            return Ok(res);
        }
        match self.name_lookup(&ast.fname.name) {
            NameLookupResult::MyLocal(name) => {
                res.push(Op::load(name));
            }
            NameLookupResult::ExternLocal(name) => {
                res.push(Op::load_non_local(name));
            }
            NameLookupResult::Global(name) => {
                res.push(Op::push_global_default(name));
            }
            NameLookupResult::NotFound => {
                return Err(CompileError::new(
                    CompileErrorType::UndefinedVariable(ast.fname.span),
                    format!("Undefined function: {}", ast.fname.name),
                ));
            }
        }
        for arg in ast.arguments.iter() {
            res.append(&mut self.compile_expr(arg)?);
        }

        res.push(Op::debug(self.create_debug_descriptor(ast.span)));

        res.push(Op::call_function(ast.arguments.len() as u8));
        Ok(res)
    }

    pub fn compile_attr_lookup(&mut self, ast: &AttrLookup) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent)?);
        let name_val = StringObject::new(RustClone::clone(&ast.attribute.name));
        res.push(Op::push_const(self.create_constant_descriptor(name_val)));

        res.push(Op::debug(self.create_debug_descriptor(ast.span)));

        res.push(Op::get_attr);
        Ok(res)
    }

    pub fn compile_method_call(&mut self, ast: &MethodCall) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent)?);
        let name_val = StringObject::new(RustClone::clone(&ast.fname.name));
        res.push(Op::push_const(self.create_constant_descriptor(name_val)));
        for arg in ast.arguments.iter() {
            res.append(&mut self.compile_expr(arg)?);
        }

        res.push(Op::debug(self.create_debug_descriptor(ast.span)));

        res.push(Op::call_method(ast.arguments.len() as u8));
        Ok(res)
    }

    pub fn compile_expr(&mut self, ast: &Expr) -> CompileResult {
        match ast {
            Expr::Variable(v) => {
                let mut res = vec![];
                res.push(Op::debug(self.create_debug_descriptor(v.span)));
                res.push(match self.name_lookup(&v.name) {
                    NameLookupResult::MyLocal(name) => Op::load(name),
                    NameLookupResult::ExternLocal(name) => Op::load_non_local(name),
                    NameLookupResult::Global(name) => Op::push_global_default(name),
                    NameLookupResult::NotFound => return Err(CompileError::new(
                        CompileErrorType::UndefinedVariable(ast.span()),
                        format!("Undefined variable: {}", v.name),
                    )),
                });
                Ok(res)
            },
            Expr::Literal(l) => self.compile_literal(l),
            Expr::ListLiteral(l) => self.compile_list_literal(l),
            Expr::SetLiteral(l) => self.compile_set_literal(l),
            Expr::TupleLiteral(t) => self.compile_tuple_literal(t),
            Expr::MethodCall(m) => self.compile_method_call(m),
            Expr::FuncCall(f) => self.compile_func_call(f),
            Expr::AttrLookup(a) => self.compile_attr_lookup(a),
            Expr::IndexedExpr(i) => self.compile_indexed_expr(i),
            Expr::SlicedExpr(s) => self.compile_sliced_expr(s),
            Expr::PostPreOp(o) => self.compile_post_pre_op(o),
        }
    }

    pub fn compile_indexed_expr(&mut self, ast: &IndexedExpr) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent)?);
        res.append(&mut self.compile_expr(&*ast.index)?);

        res.push(Op::debug(self.create_debug_descriptor(ast.span)));

        res.push(Op::index_get);
        Ok(res)
    }
    
    pub fn compile_post_pre_op(&mut self, ast: &PostPreOp) -> CompileResult {
        let mut res = vec![];

        let debug_descr = self.create_debug_descriptor(ast.span);

        res.append(&mut self.compile_expr(&ast.val.as_expr())?);

        match ast.variant {
            PPOVariant::PostIncrement => {
                res.push(Op::dup);
                res.push(Op::push_int(1));
                res.push(Op::debug(debug_descr));
                res.push(Op::add);
            },
            PPOVariant::PreIncrement => {
                res.push(Op::push_int(1));
                res.push(Op::debug(debug_descr));
                res.push(Op::add);
                res.push(Op::dup);
            },
            PPOVariant::PostDecrement => {
                res.push(Op::dup);
                res.push(Op::push_int(1));
                res.push(Op::debug(debug_descr));
                res.push(Op::sub);
            },
            PPOVariant::PreDecrement => {
                res.push(Op::push_int(1));
                res.push(Op::debug(debug_descr));
                res.push(Op::sub);
                res.push(Op::dup);
            }
        }

        res.append(&mut self.compile_assign_to(&ast.val)?);

        Ok(res)
    }

    pub fn compile_sliced_expr(&mut self, ast: &SlicedExpr) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent)?);
        match &ast.start {
            Some(start_expr) => {
                res.append(&mut self.compile_expr(&*start_expr)?);
            }
            None => {
                res.push(Op::push_void);
            }
        }
        match &ast.end {
            Some(end_expr) => {
                res.append(&mut self.compile_expr(&*end_expr)?);
            }
            None => {
                res.push(Op::push_void);
            }
        }
        match &ast.step {
            Some(step_expr) => {
                res.append(&mut self.compile_expr(&*step_expr)?);
            }
            None => {
                res.push(Op::push_void);
            }
        }
        res.push(Op::make_slice);
        Ok(res)
    }

    pub fn compile_for_loop(&mut self, ast: &ForLoop) -> CompileResult {
        let mut res = vec![];
        // Evaluate the expression for the iterator
        res.append(&mut self.compile_expr(&ast.iter)?);
        
        let debug_descr = self.create_debug_descriptor(ast.iter.span());
        res.push(Op::debug(debug_descr));

        // Turn it into an iterator
        res.push(Op::make_iter);

        // Override any variable of the appropriate name
        let local_name = self.local_name_gen();
        let cid = self.context().context_id;
        self.local_index
            .insert((cid, ast.binding.name.clone()), local_name);

        let mut body = self.compile_statement_list(&ast.body)?;

        res.push(Op::dup);
        let skip_body_offset = body.len() as u16 as i16 + 3;
        let back_to_dup_offset = -(body.len() as u16 as i16) - 4;
        res.push(Op::debug(debug_descr));
        res.push(Op::take_iter(skip_body_offset));

        res.push(Op::store(local_name));

        res.append(&mut body);

        res.push(Op::jmp(back_to_dup_offset));

        Ok(res)
    }

    pub fn compile_while_loop(&mut self, ast: &WhileLoop) -> CompileResult {
        let mut res = vec![];
        let mut cond = self.compile_expr(&ast.cond)?;
        cond.push(Op::not);
        let mut body = self.compile_statement_list(&ast.body)?;
        let skip_body = Op::cond_jmp(2 + body.len() as i16);
        let to_beginning = Op::jmp(-(body.len() as i16 + cond.len() as i16 + 1));
        res.append(&mut cond);
        res.push(skip_body);
        res.append(&mut body);
        res.push(to_beginning);
        Ok(res)
    }

    pub fn compile_if_statement(&mut self, ast: &IfStatement) -> CompileResult {
        let mut cond = self.compile_expr(&ast.condition)?;
        let mut body2 = self.compile_statement_list(&ast.then_body)?;
        let mut body1 = match ast.tail {
            Some(IfTail::ElseIf(ref ifstmt)) => self.compile_if_statement(ifstmt)?,
            Some(IfTail::Else(ref stmtlist)) => self.compile_statement_list(stmtlist)?,
            None => vec![],
        };
        let skip_body1_offset = body1.len() + 2;
        let skip_body2_offset = body2.len() + 1;
        let mut res = vec![];
        res.append(&mut cond);
        res.push(Op::cond_jmp(skip_body1_offset as i16));
        res.append(&mut body1);
        res.push(Op::jmp(skip_body2_offset as i16));
        res.append(&mut body2);
        Ok(res)
    }

    pub fn compile_case_of(&mut self, ast: &CaseOf) -> CompileResult {
        let my_local = self.local_name_gen();
        let mut res = vec![];
        let mut exit_jmp_indices: Vec<usize> = vec![];

        res.append(&mut self.compile_expr(&ast.condition)?);
        res.push(Op::store(my_local));

        for (expr, body) in ast.cases.iter() {
            let mut body = self.compile_statement_list(&body)?;
            res.push(Op::load(my_local));
            res.append(&mut self.compile_expr(&expr)?);
            res.push(Op::debug(self.create_debug_descriptor(expr.span())));
            res.push(Op::cmp_neq);
            res.push(Op::cond_jmp(body.len() as i16 + 2));
            res.append(&mut body);
            exit_jmp_indices.push(res.len());
            res.push(Op::jmp(0));
        }

        for index in exit_jmp_indices {
            res[index] = Op::jmp(res.len() as i16 - index as i16);
        }

        Ok(res)
    }

    pub fn compile_func_definition(&mut self, ast: &FuncDefinition) -> CompileResult {
        // add my_local to local_index before body
        // for recursive functions
        let my_local = self.local_name_gen();
        let cid = self.context().context_id;
        self.local_index
            .insert((cid, ast.name.name.clone()), my_local);
        
        let new_cid = self.context_id_gen();
        self.memory_manager.register_context(new_cid);
        let sub_context = CompileContext::new(new_cid, self.context().file_id);
        for arg in ast.args.iter() {
            let name = self.local_name_gen();
            self.local_index
                .insert((sub_context.context_id, arg.name.clone()), name);
        }
        
        let sub_context_id = sub_context.context_id;
        self.context_stack.push(sub_context);


        let mut func_code = vec![];
        for arg in ast.args.iter() {
            func_code.push(Op::store(
                *self
                    .local_index
                    .get(&(sub_context_id, arg.name.clone()))
                    .unwrap(),
            ));
        }

        func_code.append(&mut self.compile_statement_list(&ast.body)?);
        let finished_context = self.context_stack.pop().unwrap();
        let sub_context = GlobalContext {
            constant_descriptors: finished_context.constant_descriptors,
            debug_descriptors: finished_context.debug_symbol_descriptors,
        };
        let function_obj = Function {
            nargs: ast.args.len(),
            name: ast.name.name.clone(),
            context: Rc::new(sub_context),
            context_id: finished_context.context_id,
            least_ancestors: RwLock::new(None),
            code: func_code,
        };
        let my_descr = self.create_constant_descriptor(ObjectRef::new(function_obj));
        let mut res = vec![];
        res.push(Op::push_const_clone(my_descr));
        res.push(Op::attach_ancestors);
        res.push(Op::store(my_local));
        Ok(res)
    }

    pub fn compile_return_statement(&mut self, ast: &ReturnStatement) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&ast.ret)?);
        res.push(Op::ret);
        Ok(res)
    }

    pub fn compile_format_string(&mut self, ast: &FormatString) -> CompileResult {
        let mut res = vec![];
        let debug_descr = self.create_debug_descriptor(ast.span);
        res.push(Op::push_const(self.create_constant_descriptor(StringObject::new(ast.val.clone()))));
        for subs in ast.substitutions.iter().rev() {
            res.append(&mut self.compile_expr(subs)?);
        }
        res.push(Op::debug(debug_descr));
        res.push(Op::fmt_string(ast.substitutions.len() as u8));
        Ok(res)
    }

    pub fn compile_sh_statement(&mut self, ast: &ShStatement) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_format_string(&ast.inner)?);
        res.push(Op::debug(self.create_debug_descriptor(ast.span)));
        res.push(Op::sh);
        Ok(res)
    }

    pub fn compile_assign_to(&mut self, ast: &AssignmentLHS) -> CompileResult {
        let debug_descr = self.create_debug_descriptor(ast.as_expr().span());
        let mut res = vec![];
        match &ast {
            AssignmentLHS::Identifier(id) => {
                res.push(Op::debug(debug_descr));
                match self.name_lookup(&id.name) {
                    NameLookupResult::MyLocal(index) => {
                        res.push(Op::store(index));
                    }
                    NameLookupResult::ExternLocal(name) => {
                        res.push(Op::store_non_local(name));
                    }
                    _ => {
                        let local_name = self.local_name_gen();
                        let cid = self.context().context_id;
                        self.local_index
                            .insert(RustClone::clone(&(cid, id.name.clone())), local_name);
                        res.push(Op::store(local_name));
                    }
                }
            }
            AssignmentLHS::AttrLookup(a_lookup) => {
                res.append(&mut self.compile_expr(&a_lookup.parent)?);
                res.push(Op::push_const(self.create_constant_descriptor(StringObject::new(a_lookup.attribute.name.clone()))));
                res.push(Op::swap);
                res.push(Op::debug(debug_descr));
                res.push(Op::set_attr);
            }
            AssignmentLHS::Indexed(indexed) => {
                res.append(&mut self.compile_expr(&indexed.parent)?);
                res.push(Op::swap);
                res.append(&mut self.compile_expr(&indexed.index)?);
                res.push(Op::swap);
                res.push(Op::debug(debug_descr));
                res.push(Op::index_set);
            }
        }
        Ok(res)
    }

    pub fn compile_assignment(&mut self, ast: &Assignment) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&ast.val)?);
        res.append(&mut self.compile_assign_to(&ast.lhs)?);
        Ok(res)
    }

    pub fn compile_statement(&mut self, ast: &Statement) -> CompileResult {
        match ast {
            Statement::ForLoop(f) => self.compile_for_loop(f),
            Statement::WhileLoop(w) => self.compile_while_loop(w),
            Statement::IfStatement(i) => self.compile_if_statement(i),
            Statement::CaseOf(c) => self.compile_case_of(c),
            Statement::ReturnStatement(r) => self.compile_return_statement(r),
            Statement::ShStatement(s) => self.compile_sh_statement(s),
            Statement::FuncDefinition(f) => self.compile_func_definition(f),
            Statement::Assignment(a) => self.compile_assignment(a),
            Statement::Expr(e) => {
                let mut vals = self.compile_expr(e)?;
                vals.push(Op::pop);
                Ok(vals)
            }
        }
    }

    pub fn compile_statement_list(&mut self, ast: &StatementList) -> CompileResult {
        let mut res = vec![];
        for statement in ast.statements.iter() {
            res.append(&mut self.compile_statement(statement)?);
        }
        if *(&self.context().context_id) == 0 {
            panic!();
        }
        Ok(res)
    }
}
