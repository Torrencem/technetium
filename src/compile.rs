
use crate::ast::*;
use crate::core::*;
use crate::bytecode::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::clone::Clone as RustClone;
use std::result::Result as RustResult;

type GCDGenerator = Box<dyn FnMut() -> GlobalConstantDescriptor>;
type LocalNameGenerator = Box<dyn FnMut() -> LocalName>;

#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: CompileErrorType,
    pub help: String,
}

#[derive(Debug, Clone)]
pub enum CompileErrorType {
    UndefinedVariable,
}

impl CompileError {
    fn new(kind: CompileErrorType, help: &str) -> Self {
        CompileError { kind: kind, help: help.to_string() }
    }
}

type CompileResult = RustResult<Vec<Op>, CompileError>;

pub struct CompileContext {
    gcd_last: GlobalConstantDescriptor,
    pub constant_descriptors: HashMap<GlobalConstantDescriptor, ObjectRef>,
    local_index_last: LocalName,
    pub local_index: HashMap<String, LocalName>,
}

impl CompileContext {
    pub fn new() -> CompileContext {
        CompileContext {
            gcd_last: 0,
            constant_descriptors: HashMap::new(),
            local_index_last: 0,
            local_index: HashMap::new(),
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
}

pub trait Compilable {
    fn compile(&self, context: &mut CompileContext) -> CompileResult;
}

impl Compilable for Literal {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let descr = context.gcd_gen();
        let constant: ObjectRef = match self {
            Literal::Integer(val) => {
                IntObject::new(*val)
            },
            Literal::Float(val) => {
                FloatObject::new(*val)
            },
            Literal::Str(val) => {
                Arc::new(RustClone::clone(val))
            },
        };
        context.constant_descriptors.insert(descr, constant);
        Ok(vec![Op::push_const(descr)])
    }
}

impl Compilable for ListLiteral {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        for item in self.values.iter() {
            res.append(&mut item.compile(context)?);
        }
        res.push(Op::mklist(self.values.len() as u16));
        Ok(res)
    }
}

impl Compilable for TupleLiteral {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        for item in self.values.iter() {
            res.append(&mut item.compile(context)?);
        }
        res.push(Op::mktuple(self.values.len() as u16));
        Ok(res)
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
    res.insert("<index>".to_string(), Op::index);
    res
}

impl Compilable for FuncCall {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        let builtins = builtin_functions();
        if let Some(op) = builtins.get(&self.fname) {
            for arg in self.arguments.iter() {
                res.append(&mut arg.compile(context)?);
            }
            res.push(*op);
            return Ok(res);
        }
        let local_name = context.local_index.get(&self.fname);
        if let Some(local_name) = local_name {
            res.push(Op::load(*local_name));
        } else {
            return Err(CompileError::new(CompileErrorType::UndefinedVariable, format!("Undefined function: {}", self.fname).as_ref()));
        }
        for arg in self.arguments.iter() {
            res.append(&mut arg.compile(context)?);
        }
        res.push(Op::call_function(self.arguments.len() as u8));
        Ok(res)
    }
}

impl Compilable for AttrLookup {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.parent.compile(context)?);
        let const_descr = context.gcd_gen();
        let name_val = Arc::new(RustClone::clone(&self.attribute));
        context.constant_descriptors.insert(const_descr, name_val);
        res.push(Op::push_const(const_descr));
        res.push(Op::get_attr);
        Ok(res)
    }
}

impl Compilable for MethodCall {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.parent.compile(context)?);
        let const_descr = context.gcd_gen();
        let name_val = Arc::new(RustClone::clone(&self.call.fname));
        context.constant_descriptors.insert(const_descr, name_val);
        res.push(Op::push_const(const_descr));
        for arg in self.call.arguments.iter() {
            res.append(&mut arg.compile(context)?);
        }
        res.push(Op::call_method(self.call.arguments.len() as u8));
        Ok(res)
    }
}

impl Compilable for Expr {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        match self {
            Expr::Variable(v) => {
                let local_name = context.local_index.get(v);
                if let Some(local_name) = local_name {
                    Ok(vec![Op::load(*local_name)])
                } else {
                    return Err(CompileError::new(CompileErrorType::UndefinedVariable, format!("Undefined variable: {}", v).as_ref()));
                }
            },
            Expr::Literal(l) => l.compile(context),
            Expr::ListLiteral(l) => l.compile(context),
            Expr::TupleLiteral(t) => t.compile(context),
            Expr::MethodCall(m) => m.compile(context),
            Expr::FuncCall(f) => f.compile(context),
            Expr::AttrLookup(a) => a.compile(context),
            Expr::IndexedExpr(i) => i.compile(context),
        }
    }
}

impl Compilable for IndexedExpr {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.parent.compile(context)?);
        res.append(&mut self.index.compile(context)?);
        res.push(Op::index);
        Ok(res)
    }
}

impl Compilable for ForLoop {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        unimplemented!()
    }
}

impl Compilable for IfStatement {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut cond = self.condition.compile(context)?;
        let mut body1 = self.then_body.compile(context)?;
        let has_elif = self.else_if.is_some();
        let has_else = self.else_body.is_some();
        assert!(!(has_else && has_elif));
        let mut body2 = if let Some(ref ifstmt) = self.else_if {
            ifstmt.compile(context)?
        } else if let Some(ref stmtlist) = self.else_body {
            stmtlist.compile(context)?
        } else {
            unreachable!()
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
}

impl Compilable for CaseOf {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        unimplemented!()
    }
}

impl Compilable for FuncDefinition {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut sub_context = CompileContext::new();
        for arg in self.args.iter() {
            let name = sub_context.local_name_gen();
            sub_context.local_index.insert(arg.clone(), name);
        }
        let code = self.body.compile(&mut sub_context);
        let sub_context = GlobalContext {
            constant_descriptors: sub_context.constant_descriptors,
        };
        let function_obj = Function {
            nargs: self.args.len(),
            name: self.name.clone(),
            context: Arc::new(sub_context),
            code: code?,
        };
        let my_descr = context.gcd_gen();
        context.constant_descriptors.insert(my_descr, Arc::new(function_obj));
        let my_local = context.local_name_gen();
        context.local_index.insert(self.name.clone(), my_local);
        let mut res = vec![];
        res.push(Op::push_const(my_descr));
        res.push(Op::store(my_local));
        Ok(res)
    }
}

impl Compilable for ReturnStatement {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.ret.compile(context)?);
        res.push(Op::ret);
        Ok(res)
    }
}

impl Compilable for Assignment {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        res.append(&mut self.val.compile(context)?);
        let local_name = context.local_name_gen();
        context.local_index.insert(RustClone::clone(&self.name), local_name);
        res.push(Op::store(local_name));
        Ok(res)
    }
}

impl Compilable for Statement {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        match self {
            Statement::ForLoop(f) => f.compile(context),
            Statement::IfStatement(i) => i.compile(context),
            Statement::CaseOf(c) => c.compile(context),
            Statement::ReturnStatement(r) => r.compile(context),
            Statement::FuncDefinition(f) => f.compile(context),
            Statement::Assignment(a) => a.compile(context),
            Statement::Expr(e) => e.compile(context),
        }
    }
}

impl Compilable for StatementList {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        let mut res = vec![];
        for statement in self.statements.iter() {
            res.append(&mut statement.compile(context)?);
        }
        Ok(res)
    }
}
