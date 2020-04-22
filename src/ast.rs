
#[derive(Clone, Debug)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Str(String),
}

#[derive(Clone, Debug)]
pub struct ListLiteral {
    pub values: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct TupleLiteral {
    pub values: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct FuncCall {
    pub fname: String,
    pub arguments: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub struct AttrLookup {
    pub parent: Box<Expr>,
    pub attribute: String,
}

#[derive(Clone, Debug)]
pub struct MethodCall {
    pub parent: Box<Expr>,
    pub call: FuncCall,
}

#[derive(Clone, Debug)]
pub struct IndexedExpr {
    pub parent: Box<Expr>,
    pub index: Box<Expr>,
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
pub struct Pattern(pub String);

#[derive(Clone, Debug)]
pub struct ForLoop {
    pub binding: Pattern,
    pub iter: Expr,
    pub body: StatementList,
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Expr,
    pub then_body: StatementList,
    pub else_if: Option<Box<IfStatement>>,
    pub else_body: Option<StatementList>,
}

#[derive(Clone, Debug)]
pub struct CaseOf {
    pub condition: Expr,
    pub cases: Vec<(Expr, StatementList)>,
}

#[derive(Clone, Debug)]
pub struct FuncDefinition {
    pub name: String,
    pub args: Vec<String>,
    pub body: StatementList,
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub ret: Expr,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub name: String,
    pub val: Expr,
}

#[derive(Clone, Debug)]
pub enum Statement {
    ForLoop(ForLoop),
    IfStatement(IfStatement),
    CaseOf(CaseOf),
    ReturnStatement(ReturnStatement),
    FuncDefinition(FuncDefinition),
    Assignment(Assignment),
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}
