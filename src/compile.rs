
use crate::ast::*;
use crate::core::*;
use crate::bytecode::*;
use crate::standard::Default_Namespace_Descriptors;
use std::collections::HashMap;
use std::sync::Arc;
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
    gcd_last: GlobalConstantDescriptor,
    pub constant_descriptors: HashMap<GlobalConstantDescriptor, ObjectRef>,
    local_index_last: LocalName,
    pub local_index: HashMap<String, LocalName>,
    dcd_last: DebugSpanDescriptor,
    pub debug_span_descriptors: HashMap<DebugSpanDescriptor, Span>,
}

impl CompileContext {
    pub fn new() -> CompileContext {
        CompileContext {
            gcd_last: 0,
            constant_descriptors: HashMap::new(),
            local_index_last: 0,
            local_index: HashMap::new(),
            dcd_last: 0,
            debug_span_descriptors: HashMap::new(),
        }
    }

    pub fn gcd_gen(&mut self) -> GlobalConstantDescriptor {
        let old = self.gcd_last;
        self.gcd_last += 1;
        old
    }
    
    pub fn local_name_gen(&mut self) -> LocalName {
        let old = self.local_index_last;
        self.local_index_last += 1;
        old
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
}

impl CompileManager {
    pub fn compile_literal(&self, ast: &Literal, context: &mut CompileContext) -> CompileResult {
        let descr = context.gcd_gen();
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
        context.constant_descriptors.insert(descr, constant);
        Ok(vec![Op::push_const(descr)])
    }

    pub fn compile_list_literal(&self, ast: &ListLiteral, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        for item in ast.values.iter() {
            res.append(&mut self.compile_expr(item, context)?);
        }
        res.push(Op::mklist(ast.values.len() as u16));
        Ok(res)
    }
    
    pub fn compile_tuple_literal(&self, ast: &TupleLiteral, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        for item in ast.values.iter() {
            res.append(&mut self.compile_expr(item, context)?);
        }
        res.push(Op::mktuple(ast.values.len() as u16));
        Ok(res)
    }

    pub fn compile_func_call(&self, ast: &FuncCall, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        let builtins = builtin_functions();
        if let Some(op) = builtins.get(&ast.fname.name) {
            for arg in ast.arguments.iter() {
                res.append(&mut self.compile_expr(arg, context)?);
            }
            res.push(*op);
            return Ok(res);
        }
        let local_name = context.local_index.get(&ast.fname.name);
        if let Some(local_name) = local_name {
            res.push(Op::load(*local_name));
        } else if let Some(global_name) = Default_Namespace_Descriptors.get(&ast.fname.name) {
            res.push(Op::push_global_default(*global_name));
        } else {
            return Err(CompileError::new(CompileErrorType::UndefinedVariable(ast.fname.span), format!("Undefined function: {}", ast.fname.name).as_ref()));
        }
        for arg in ast.arguments.iter() {
            res.append(&mut self.compile_expr(arg, context)?);
        }

        let debug_descr = context.dsd_gen();
        context.debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::weak_debug(debug_descr));

        res.push(Op::call_function(ast.arguments.len() as u8));
        Ok(res)
    }

    pub fn compile_attr_lookup(&self, ast: &AttrLookup, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent, context)?);
        let const_descr = context.gcd_gen();
        let name_val = Arc::new(RustClone::clone(&ast.attribute.name));
        context.constant_descriptors.insert(const_descr, name_val);
        res.push(Op::push_const(const_descr));

        let debug_descr = context.dsd_gen();
        context.debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::debug(debug_descr));

        res.push(Op::get_attr);
        Ok(res)
    }

    pub fn compile_method_call(&self, ast: &MethodCall, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent, context)?);
        let const_descr = context.gcd_gen();
        let name_val = Arc::new(RustClone::clone(&ast.fname.name));
        context.constant_descriptors.insert(const_descr, name_val);
        res.push(Op::push_const(const_descr));
        for arg in ast.arguments.iter() {
            res.append(&mut self.compile_expr(arg, context)?);
        }
        
        let debug_descr = context.dsd_gen();
        context.debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::weak_debug(debug_descr));

        res.push(Op::call_method(ast.arguments.len() as u8));
        Ok(res)
    }

    pub fn compile_expr(&self, ast: &Expr, context: &mut CompileContext) -> CompileResult {
        match ast {
            Expr::Variable(v) => {
                let global_name = Default_Namespace_Descriptors.get(&v.name);
                if let Some(global_name) = global_name {
                    Ok(vec![Op::push_global_default(*global_name)])
                } else {
                    let local_name = context.local_index.get(&v.name);
                    if let Some(local_name) = local_name {
                        Ok(vec![Op::load(*local_name)])
                    } else {
                        return Err(CompileError::new(CompileErrorType::UndefinedVariable(v.span), format!("Undefined variable: {}", v.name).as_ref()));
                    }
                }
            },
            Expr::Literal(l) => self.compile_literal(l, context),
            Expr::ListLiteral(l) => self.compile_list_literal(l, context),
            Expr::TupleLiteral(t) => self.compile_tuple_literal(t, context),
            Expr::MethodCall(m) => self.compile_method_call(m, context),
            Expr::FuncCall(f) => self.compile_func_call(f, context),
            Expr::AttrLookup(a) => self.compile_attr_lookup(a, context),
            Expr::IndexedExpr(i) => self.compile_indexed_expr(i, context),
        }
    }

    pub fn compile_indexed_expr(&self, ast: &IndexedExpr, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&*ast.parent, context)?);
        res.append(&mut self.compile_expr(&*ast.index, context)?);

        let debug_descr = context.dsd_gen();
        context.debug_span_descriptors.insert(debug_descr, ast.span);
        res.push(Op::debug(debug_descr));

        res.push(Op::index);
        Ok(res)
    }

    pub fn compile_for_loop(&self, ast: &ForLoop, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        // Evaluate the expression for the iterator
        res.append(&mut self.compile_expr(&ast.iter, context)?);

        let debug_descr = context.dsd_gen();
        context.debug_span_descriptors.insert(debug_descr, ast.iter.span());
        res.push(Op::debug(debug_descr));

        // Turn it into an iterator
        res.push(Op::make_iter);
        
        // Override any variable of the appropriate name
        let local_name = context.local_name_gen();
        context.local_index.insert(ast.binding.name.clone(), local_name);

        let mut body = self.compile_statement_list(&ast.body, context)?;
        
        res.push(Op::dup);
        let skip_body_offset = body.len() as u16 as i16 + 3;
        let back_to_dup_offset = -(body.len() as u16 as i16) - 3;
        res.push(Op::debug(debug_descr));
        res.push(Op::take_iter(skip_body_offset));

        res.push(Op::store(local_name));

        res.append(&mut body);

        res.push(Op::jmp(back_to_dup_offset));
        
        Ok(res)
    }

    pub fn compile_while_loop(&self, ast: &WhileLoop, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        let mut cond = self.compile_expr(&ast.cond, context)?;
        let mut body = self.compile_statement_list(&ast.body, context)?;
        let skip_body = Op::cond_jmp(2 + body.len() as i16);
        let to_beginning = Op::jmp(-(body.len() as i16 + cond.len() as i16 + 1));
        res.append(&mut cond);
        res.push(skip_body);
        res.append(&mut body);
        res.push(to_beginning);
        Ok(res)
    }

    pub fn compile_if_statement(&self, ast: &IfStatement, context: &mut CompileContext) -> CompileResult {
        let mut cond = self.compile_expr(&ast.condition, context)?;
        let mut body1 = self.compile_statement_list(&ast.then_body, context)?;
        let mut body2 = match ast.tail {
            Some(IfTail::ElseIf(ref ifstmt)) => {
                self.compile_if_statement(ifstmt, context)?
            },
            Some(IfTail::Else(ref stmtlist)) => {
                self.compile_statement_list(stmtlist, context)?
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

    pub fn compile_case_of(&self, ast: &CaseOf, context: &mut CompileContext) -> CompileResult {
        unimplemented!()
    }

    pub fn compile_func_definition(&self, ast: &FuncDefinition, context: &mut CompileContext) -> CompileResult {
        let mut sub_context = CompileContext::new();
        for arg in ast.args.iter() {
            let name = sub_context.local_name_gen();
            sub_context.local_index.insert(arg.name.clone(), name);
        }
        let code = self.compile_statement_list(&ast.body, &mut sub_context);
        let sub_context = GlobalContext {
            constant_descriptors: sub_context.constant_descriptors,
            debug_descriptors: sub_context.debug_span_descriptors,
        };
        let function_obj = Function {
            nargs: ast.args.len(),
            name: ast.name.name.clone(),
            context: Arc::new(sub_context),
            code: code?,
        };
        let my_descr = context.gcd_gen();
        context.constant_descriptors.insert(my_descr, Arc::new(function_obj));
        let my_local = context.local_name_gen();
        context.local_index.insert(ast.name.name.clone(), my_local);
        let mut res = vec![];
        res.push(Op::push_const(my_descr));
        res.push(Op::store(my_local));
        Ok(res)
    }

    pub fn compile_return_statement(&self, ast: &ReturnStatement, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&ast.ret, context)?);
        res.push(Op::ret);
        Ok(res)
    }

    pub fn compile_assignment(&self, ast: &Assignment, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.compile_expr(&ast.val, context)?);
        if let Some(local_name) = context.local_index.get(&ast.name.name) {
            res.push(Op::store(*local_name));
            Ok(res)
        } else {
            let local_name = context.local_name_gen();
            context.local_index.insert(RustClone::clone(&ast.name.name), local_name);
            res.push(Op::store(local_name));
            Ok(res)
        }
    }

    pub fn compile_statement(&self, ast: &Statement, context: &mut CompileContext) -> CompileResult {
        match ast {
            Statement::ForLoop(f) => self.compile_for_loop(f, context),
            Statement::WhileLoop(w) => self.compile_while_loop(w, context),
            Statement::IfStatement(i) => self.compile_if_statement(i, context),
            Statement::CaseOf(c) => self.compile_case_of(c, context),
            Statement::ReturnStatement(r) => self.compile_return_statement(r, context),
            Statement::FuncDefinition(f) => self.compile_func_definition(f, context),
            Statement::Assignment(a) => self.compile_assignment(a, context),
            Statement::Expr(e) => self.compile_expr(e, context),
        }
    }

    pub fn compile_statement_list(&self, ast: &StatementList, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        for statement in ast.statements.iter() {
            res.append(&mut self.compile_statement(statement, context)?);
        }
        Ok(res)
    }
}
