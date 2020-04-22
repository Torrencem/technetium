
use codespan::Span;
use crate::compile::*;

#[derive(Clone, Debug)]
pub struct ASTNode<T> {
    pub span: Span,
    pub inner: T,
}

impl<T> ASTNode<T> {
    pub fn new(inner: T, left: usize, right: usize) -> Self {
        Self { inner, span: Span::new(left as u32, right as u32) }
    }
}

impl<T: Compilable> Compilable for ASTNode<T> {
    fn compile(&self, context: &mut CompileContext) -> CompileResult {
        self.inner.compile(context)
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Str(String),
}

#[derive(Clone, Debug)]
pub struct ListLiteral {
    pub values: Vec<ASTNode<Expr>>,
}

#[derive(Clone, Debug)]
pub struct TupleLiteral {
    pub values: Vec<ASTNode<Expr>>,
}

#[derive(Clone, Debug)]
pub struct FuncCall {
    pub fname: String,
    pub arguments: Vec<ASTNode<Expr>>,
}

#[derive(Clone, Debug)]
pub struct AttrLookup {
    pub parent: Box<ASTNode<Expr>>,
    pub attribute: ASTNode<String>,
}

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub parent: Box<ASTNode<Expr>>,
    pub call: FuncCall,
}

#[derive(Clone, Debug)]
pub struct IndexedExpr {
    pub parent: Box<ASTNode<Expr>>,
    pub index: Box<ASTNode<Expr>>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(String),
    Literal(Literal),
    ListLiteral(ListLiteral),
    TupleLiteral(TupleLiteral),
    MethodCall(MethodCall),
    FuncCall(FuncCall),
    AttrLookup(AttrLookup),
    IndexedExpr(IndexedExpr),
}

#[derive(Clone, Debug)]
pub struct ForLoop {
    pub binding: ASTNode<String>,
    pub iter: ASTNode<Expr>,
    pub body: ASTNode<StatementList>,
}

#[derive(Clone, Debug)]
pub struct WhileLoop {
    pub cond: ASTNode<Expr>,
    pub body: ASTNode<StatementList>,
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: ASTNode<Expr>,
    pub then_body: ASTNode<StatementList>,
    pub else_if: Option<Box<IfStatement>>,
    pub else_body: Option<ASTNode<StatementList>>,
}

#[derive(Clone, Debug)]
pub struct CaseOf {
    pub condition: ASTNode<Expr>,
    pub cases: Vec<(ASTNode<Expr>, ASTNode<StatementList>)>,
}

#[derive(Clone, Debug)]
pub struct FuncDefinition {
    pub name: ASTNode<String>,
    pub args: Vec<ASTNode<String>>,
    pub body: ASTNode<StatementList>,
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub ret: ASTNode<Expr>,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub name: ASTNode<String>,
    pub val: ASTNode<Expr>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    IfStatement(IfStatement),
    CaseOf(CaseOf),
    ReturnStatement(ReturnStatement),
    FuncDefinition(FuncDefinition),
    Assignment(Assignment),
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub struct StatementList {
    pub statements: Vec<ASTNode<Statement>>,
}
