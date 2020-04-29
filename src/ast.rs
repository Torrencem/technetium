
use codespan::Span;
use crate::compile::*;

use crate::lexer::Lexer;
use crate::script;
use lalrpop_util;
use crate::lexer;

#[derive(Clone, Debug)]
pub enum Literal {
    Integer(i64, Span),
    Float(f64, Span),
    Str(String, Span),
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::Integer(_, s) => *s,
            Literal::Float(_, s) => *s,
            Literal::Str(_, s) => *s,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ListLiteral {
    pub span: Span,
    pub values: Vec<Expr>,
}

impl ListLiteral {
    pub fn new(values: Vec<Expr>, l: usize, r: usize) -> Self {
        ListLiteral { 
            span: Span::new(l as u32, r as u32),
            values
        }
    }
}

#[derive(Clone, Debug)]
pub struct TupleLiteral {
    pub span: Span,
    pub values: Vec<Expr>,
}

impl TupleLiteral {
    pub fn new(values: Vec<Expr>, l: usize, r: usize) -> Self {
        TupleLiteral { 
            span: Span::new(l as u32, r as u32),
            values
        }
    }
}

#[derive(Clone, Debug)]
pub struct FuncCall {
    pub span: Span,
    pub fname: Identifier,
    pub arguments: Vec<Expr>,
}

impl FuncCall {
    pub fn new(fname: Identifier, arguments: Vec<Expr>, l: usize, r: usize) -> Self {
        FuncCall {
            span: Span::new(l as u32, r as u32),
            fname, arguments
        }
    }
}

#[derive(Clone, Debug)]
pub struct AttrLookup {
    pub span: Span,
    pub parent: Box<Expr>,
    pub attribute: Identifier,
}

impl AttrLookup {
    pub fn new(parent: Expr, attribute: Identifier, l: usize, r: usize) -> Self {
        AttrLookup {
            span: Span::new(l as u32, r as u32),
            parent: Box::new(parent),
            attribute,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub span: Span,
    pub parent: Box<Expr>,
    pub fname: Identifier,
    pub arguments: Vec<Expr>,
}

impl MethodCall {
    pub fn new(parent: Expr, fname: Identifier, arguments: Vec<Expr>, l: usize, r: usize) -> Self {
        MethodCall {
            span: Span::new(l as u32, r as u32),
            parent: Box::new(parent),
            fname, arguments
        }
    }
}

#[derive(Clone, Debug)]
pub struct IndexedExpr {
    pub span: Span,
    pub parent: Box<Expr>,
    pub index: Box<Expr>,
}

impl IndexedExpr {
    pub fn new(parent: Expr, index: Expr, l: usize, r: usize) -> Self {
        IndexedExpr {
            span: Span::new(l as u32, r as u32),
            parent: Box::new(parent),
            index: Box::new(index),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(Identifier),
    Literal(Literal),
    ListLiteral(ListLiteral),
    TupleLiteral(TupleLiteral),
    MethodCall(MethodCall),
    FuncCall(FuncCall),
    AttrLookup(AttrLookup),
    IndexedExpr(IndexedExpr),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Variable(i) => i.span,
            Expr::Literal(l) => l.span(),
            Expr::ListLiteral(l) => l.span,
            Expr::TupleLiteral(t) => t.span,
            Expr::MethodCall(m) => m.span,
            Expr::FuncCall(f) => f.span,
            Expr::AttrLookup(a) => a.span,
            Expr::IndexedExpr(e) => e.span,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ForLoop {
    pub span: Span,
    pub binding: Identifier,
    pub iter: Expr,
    pub body: StatementList,
}

impl ForLoop {
    pub fn new(binding: Identifier, iter: Expr, body: StatementList, l: usize, r: usize) -> Self {
        ForLoop {
            span: Span::new(l as u32, r as u32),
            binding, iter, body
        }
    }
}

#[derive(Clone, Debug)]
pub struct WhileLoop {
    pub span: Span,
    pub cond: Expr,
    pub body: StatementList,
}

impl WhileLoop {
    pub fn new(cond: Expr, body: StatementList, l: usize, r: usize) -> Self {
        WhileLoop {
            span: Span::new(l as u32, r as u32),
            cond, body
        }
    }
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub span: Span,
    pub condition: Expr,
    pub then_body: StatementList,
    pub tail: Option<IfTail>,
}

#[derive(Clone, Debug)]
pub enum IfTail {
    ElseIf(Box<IfStatement>),
    Else(StatementList)
}

impl IfStatement {
    pub fn new(condition: Expr, then_body: StatementList, tail: Option<IfTail>, l: usize, r: usize) -> Self {
        IfStatement {
            span: Span::new(l as u32, r as u32),
            condition, then_body, tail
        }
    }
}

#[derive(Clone, Debug)]
pub struct CaseOf {
    pub span: Span,
    pub condition: Expr,
    pub cases: Vec<(Expr, StatementList)>,
}

impl CaseOf {
    pub fn new(condition: Expr, cases: Vec<(Expr, StatementList)>, l: usize, r: usize) -> Self {
        CaseOf {
            span: Span::new(l as u32, r as u32),
            condition, cases,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FuncDefinition {
    pub span: Span,
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: StatementList,
}

impl FuncDefinition {
    pub fn new(name: Identifier, args: Vec<Identifier>, body: StatementList, l: usize, r: usize) -> Self {
        FuncDefinition {
            span: Span::new(l as u32, r as u32),
            name, args, body
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub span: Span,
    pub ret: Expr,
}

impl ReturnStatement {
    pub fn new(ret: Expr, l: usize, r: usize) -> Self {
        ReturnStatement {
            span: Span::new(l as u32, r as u32),
            ret
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShStatement {
    pub span: Span,
    pub inner: FormatString,
}

#[derive(Clone, Debug)]
pub struct FormatString {
    pub span: Span,
    pub val: String,
    pub substitutions: Vec<Expr>,
}

impl FormatString {
    pub fn new(val: String, substitutions: Vec<String>, l: usize, r: usize) -> Self {
        let mut subs = vec![];
        for s in substitutions.iter() {
            let lexer = Lexer::new(s.as_ref());
            // TODO: Figure out error handling here. Propogating up is difficult because execution
            // is in the parser, which isn't expecting and can't properly handle a Result<Self, ParseError> here
            subs.push(script::ExprParser::new().parse(lexer).expect("Error parsing expression in {} interpolation!"));
        }
        FormatString {
            span: Span::new(l as u32, r as u32),
            val,
            substitutions: subs,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssignmentLHS {
    Identifier(Identifier),
    AttrLookup(AttrLookup),
    Indexed(IndexedExpr),
}

impl AssignmentLHS {
    pub fn as_expr(&self) -> Expr {
        match self {
            AssignmentLHS::Identifier(id) => Expr::Variable(id.clone()),
            AssignmentLHS::AttrLookup(al) => Expr::AttrLookup(al.clone()),
            AssignmentLHS::Indexed(id) => Expr::IndexedExpr(id.clone()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub span: Span,
    pub lhs: AssignmentLHS,
    pub val: Expr,
}

impl Assignment {
    pub fn new(lhs: AssignmentLHS, val: Expr, l: usize, r: usize) -> Self {
        Assignment {
            span: Span::new(l as u32, r as u32),
            lhs, val
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    IfStatement(IfStatement),
    CaseOf(CaseOf),
    ReturnStatement(ReturnStatement),
    ShStatement(ShStatement),
    FuncDefinition(FuncDefinition),
    Assignment(Assignment),
    Expr(Expr),
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::ForLoop(f) => f.span,
            Statement::WhileLoop(w) => w.span,
            Statement::IfStatement(i) => i.span,
            Statement::CaseOf(c) => c.span,
            Statement::ReturnStatement(r) => r.span,
            Statement::ShStatement(s) => s.span,
            Statement::FuncDefinition(f) => f.span,
            Statement::Assignment(a) => a.span,
            Statement::Expr(e) => e.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StatementList {
    pub span: Span,
    pub statements: Vec<Statement>,
}

impl StatementList {
    pub fn new(statements: Vec<Statement>, l: usize, r: usize) -> Self {
        StatementList {
            span: Span::new(l as u32, r as u32),
            statements
        }
    }
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub span: Span,
    pub name: String,
}

impl Identifier {
    pub fn new(name: String, l: usize, r: usize) -> Self {
        Identifier {
            span: Span::new(l as u32, r as u32),
            name
        }
    }
}
