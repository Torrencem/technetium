
use crate::ast::*;
use crate::core::*;
use crate::bytecode::*;
use crate::standard::Default_Namespace_Descriptors;
use crate::standard::STANDARD_CONTEXT_ID;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::clone::Clone as RustClone;
use std::result::Result as RustResult;
use codespan::{Span, FileId};
use codespan_reporting::diagnostic::{Diagnostic, Label};

// type GCDGenerator = Box<dyn FnMut() -> GlobalConstantDescriptor>;
// type LocalNameGenerator = Box<dyn FnMut() -> LocalName>;
// type DSDGenerator = Box<dyn FnMut() -> DebugSpanDescriptor>;

#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: CompileErrorType,
    pub help: String,
}

#[derive(Debug, Clone)]
pub enum CompileErrorType {
    UndefinedVariable(Span),
}

impl CompileError {
    pub fn new(kind: CompileErrorType, help: &str) -> Self {
        CompileError { kind: kind, help: help.to_string() }
    }

    pub fn as_diagnostic<FileId>(&self, fileid: FileId) -> Diagnostic<FileId> {
        match self.kind {
            CompileErrorType::UndefinedVariable(span) => Diagnostic::error()
                .with_message(self.help.clone())
                .with_labels(vec![
                    Label::primary(fileid, span).with_message("Undefined variable"),
                ]),
        }
    }
}

pub type CompileResult = RustResult<Vec<Op>, CompileError>;

pub struct CompileContext {
    context_id: ContextId,
    gcd_last: u16,
    pub constant_descriptors: HashMap<GlobalConstantDescriptor, ObjectRef>,
    dcd_last: DebugSpanDescriptor,
    pub debug_span_descriptors: HashMap<DebugSpanDescriptor, Span>,
}

impl CompileContext {
    pub fn new(context_id: ContextId) -> CompileContext {
        CompileContext {
            context_id,
            gcd_last: 0,
            constant_descriptors: HashMap::new(),
            dcd_last: 0,
            debug_span_descriptors: HashMap::new(),
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
    res.insert("<index>".to_string(), Op::index);
    res
}

pub struct CompileManager {
    pub context_stack: Vec<CompileContext>,
    local_index_last: LocalName,
    pub local_index: HashMap<(ContextId, String), LocalName>,
    context_id_last: ContextId,
}

pub enum NameLookupResult {
    MyLocal(LocalName),
    ExternLocal(NonLocalUnmappedName),
    Global(GlobalConstantDescriptor),
    NotFound,
}

impl CompileManager {
    pub fn new() -> Self {
        CompileManager {
            context_stack: vec![CompileContext::new(STANDARD_CONTEXT_ID + 1)],
            local_index_last: 0,
            local_index: HashMap::new(),
            context_id_last: STANDARD_CONTEXT_ID + 2,
        }
    }

    pub fn context_id_gen(&mut self) -> ContextId {
        let old = self.context_id_last;
        self.context_id_last += 1;
        old
    }

    pub fn context(&mut self) -> &mut CompileContext {
        self.context_stack.last_mut().unwrap()
    }
   
    pub fn local_name_gen(&mut self) -> LocalName {
        let old = self.local_index_last;
        self.local_index_last += 1;
        old
    }

    pub fn name_lookup(&self, name: &String) -> NameLookupResult {
        let mut first = true;
        for context in self.context_stack.iter().rev() {
            if let Some(local_index) = self.local_index.get(&(context.context_id, name.clone())) {
                if first {
                    return NameLookupResult::MyLocal(*local_index);
                } else {
                    return NameLookupResult::ExternLocal((context.context_id, *local_index));
                }
            }
            if first {
                first = false;
            }
        }
        if let Some(global_index) = Default_Namespace_Descriptors.get(name) {
            NameLookupResult::Global(*global_index)
        } else {
            NameLookupResult::NotFound
        }
    }

    pub fn compile_literal(&mut self, ast: &Literal) -> CompileResult {
        let descr = self.context().gcd_gen();
        let constant: ObjectRef = match ast {
            Literal::Integer(val, _) => {
                IntObject::new(*val)
            },
            Literal::Float(val, _) => {
                FloatObject::new(*val)
            },
            Literal::Str(val, _) => {
                Arc::new(RustClone::clone(val))
            },
        };
        self.context().constant_descriptors.insert(descr, constant);
        Ok(vec![Op::push_const(descr)])
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

    pub fn compile_func_call(&mut self, ast: &FuncCall) -> CompileResult {
        let mut res = vec![];
        let builtins = builtin_functions();
        if let Some(op) = builtins.get(&ast.fname.name) {
            for arg in ast.arguments.iter() {
                res.append(&mut self.compile_expr(arg)?);
            }
            res.push(*op);
            return Ok(res);
        }
        match self.name_lookup(&ast.fname.name) {
            NameLookupResult::MyLocal(name) => {
                res.push(Op::load(name));
            },
            NameLookupResult::ExternLocal(name) => {
                res.push(Op::load_non_local(name));
            },
            NameLookupResult::Global(name) => {
                res.push(Op::push_global_default(name));
            },
            NameLookupResult::NotFound => {
            return Err(CompileError::new(CompileErrorType::UndefinedVariable(ast.fname.span), format!("Undefined function: {}", ast.fname.name).as_ref()));
            },
        }
        for arg in ast.arguments.iter() {
            res.append(&mut self.compile_expr(arg)?);
        }

        let debug_descr = self.context().dsd_gen();
        self.context().debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::weak_debug(debug_descr));

        res.push(Op::call_function(ast.arguments.len() as u8));
        Ok(res)
    }

    pub fn compile_attr_lookup(&mut self, ast: &AttrLookup) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent)?);
        let const_descr = self.context().gcd_gen();
        let name_val = Arc::new(RustClone::clone(&ast.attribute.name));
        self.context().constant_descriptors.insert(const_descr, name_val);
        res.push(Op::push_const(const_descr));

        let debug_descr = self.context().dsd_gen();
        self.context().debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::debug(debug_descr));

        res.push(Op::get_attr);
        Ok(res)
    }

    pub fn compile_method_call(&mut self, ast: &MethodCall) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent)?);
        let const_descr = self.context().gcd_gen();
        let name_val = Arc::new(RustClone::clone(&ast.fname.name));
        self.context().constant_descriptors.insert(const_descr, name_val);
        res.push(Op::push_const(const_descr));
        for arg in ast.arguments.iter() {
            res.append(&mut self.compile_expr(arg)?);
        }
        
        let debug_descr = self.context().dsd_gen();
        self.context().debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::weak_debug(debug_descr));

        res.push(Op::call_method(ast.arguments.len() as u8));
        Ok(res)
    }

    pub fn compile_expr(&mut self, ast: &Expr) -> CompileResult {
        match ast {
            Expr::Variable(v) => {
                match self.name_lookup(&v.name) {
                    NameLookupResult::MyLocal(name) => {
                        Ok(vec![Op::load(name)])
                    },
                    NameLookupResult::ExternLocal(name) => {
                        Ok(vec![Op::load_non_local(name)])
                    },
                    NameLookupResult::Global(name) => {
                        Ok(vec![Op::push_global_default(name)])
                    },
                    NameLookupResult::NotFound => {
                        Err(CompileError::new(CompileErrorType::UndefinedVariable(ast.span()), format!("Undefined variable: {}", v.name).as_ref()))
                    },
                }
            },
            Expr::Literal(l) => self.compile_literal(l),
            Expr::ListLiteral(l) => self.compile_list_literal(l),
            Expr::TupleLiteral(t) => self.compile_tuple_literal(t),
            Expr::MethodCall(m) => self.compile_method_call(m),
            Expr::FuncCall(f) => self.compile_func_call(f),
            Expr::AttrLookup(a) => self.compile_attr_lookup(a),
            Expr::IndexedExpr(i) => self.compile_indexed_expr(i),
        }
    }

    pub fn compile_indexed_expr(&mut self, ast: &IndexedExpr) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent)?);
        res.append(&mut self.compile_expr(&*ast.index)?);

        let debug_descr = self.context().dsd_gen();
        self.context().debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::debug(debug_descr));

        res.push(Op::index);
        Ok(res)
    }

    pub fn compile_for_loop(&mut self, ast: &ForLoop) -> CompileResult {
        let mut res = vec![];
        // Evaluate the expression for the iterator
        res.append(&mut self.compile_expr(&ast.iter)?);

        let debug_descr = self.context().dsd_gen();
        self.context().debug_span_descriptors.insert(debug_descr, ast.iter.span());
        res.push(Op::debug(debug_descr));

        // Turn it into an iterator
        res.push(Op::make_iter);
        
        // Override any variable of the appropriate name
        let local_name = self.local_name_gen();
        let cid = self.context().context_id;
        self.local_index.insert((cid, ast.binding.name.clone()), local_name);

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
            Some(IfTail::ElseIf(ref ifstmt)) => {
                self.compile_if_statement(ifstmt)?
            },
            Some(IfTail::Else(ref stmtlist)) => {
                self.compile_statement_list(stmtlist)?
            },
            None => {
                vec![]
            }
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
        unimplemented!()
    }

    pub fn compile_func_definition(&mut self, ast: &FuncDefinition) -> CompileResult {
        // add my_local to local_index before body
        // for recursive functions
        let my_local = self.local_name_gen();
        let cid = self.context().context_id;
        self.local_index.insert((cid, ast.name.name.clone()), my_local);

        let mut sub_context = CompileContext::new(self.context_id_gen());
        for arg in ast.args.iter() {
            let name = self.local_name_gen();
            self.local_index.insert((sub_context.context_id, arg.name.clone()), name);
        }
        let sub_context_id = sub_context.context_id;
        self.context_stack.push(sub_context);

        let mut func_code = vec![];
        for arg in ast.args.iter() {
            func_code.push(Op::store(*self.local_index.get(&(sub_context_id, arg.name.clone())).unwrap()));
        }

        func_code.append(&mut self.compile_statement_list(&ast.body)?);
        let finished_context = self.context_stack.pop().unwrap();
        let sub_context = GlobalContext {
            constant_descriptors: finished_context.constant_descriptors,
            debug_descriptors: finished_context.debug_span_descriptors,
        };
        let function_obj = Function {
            nargs: ast.args.len(),
            name: ast.name.name.clone(),
            context: Arc::new(sub_context),
            context_id: finished_context.context_id,
            least_ancestors: Mutex::new(None),
            code: func_code,
        };
        let my_descr = self.context().gcd_gen();
        self.context().constant_descriptors.insert(my_descr, Arc::new(function_obj));
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

    pub fn compile_assignment(&mut self, ast: &Assignment) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&ast.val)?);
        match self.name_lookup(&ast.name.name) {
            NameLookupResult::MyLocal(index) => {
                res.push(Op::store(index));
                Ok(res)
            },
            NameLookupResult::ExternLocal(name) => {
                res.push(Op::store_non_local(name));
                Ok(res)
            },
            _ => {
                let local_name = self.local_name_gen();
                let cid = self.context().context_id;
                self.local_index.insert(RustClone::clone(&(cid, ast.name.name.clone())), local_name);
                res.push(Op::store(local_name));
                Ok(res)
            }
        }
    }

    pub fn compile_statement(&mut self, ast: &Statement) -> CompileResult {
        match ast {
            Statement::ForLoop(f) => self.compile_for_loop(f),
            Statement::WhileLoop(w) => self.compile_while_loop(w),
            Statement::IfStatement(i) => self.compile_if_statement(i),
            Statement::CaseOf(c) => self.compile_case_of(c),
            Statement::ReturnStatement(r) => self.compile_return_statement(r),
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
