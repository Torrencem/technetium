
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
pub struct Sh {
    pub literal: String,
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
    ShDefinition(Sh),
}

#[derive(Clone, Debug)]
pub struct Pattern(String);

#[derive(Clone, Debug)]
pub struct ForLoop {
    pub binding: Pattern,
    pub iter: Expr,
    pub body: StatementList,
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    condition: Expr,
    then_body: StatementList,
    else_body: StatementList,
}

#[derive(Clone, Debug)]
pub struct CaseOf {
    condition: Expr,
    cases: Vec<(Expr, StatementList)>,
}

#[derive(Clone, Debug)]
pub struct FuncDefinition {
    name: String,
    args: Vec<String>,
    body: StatementList,
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    ret: Expr,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    name: String,
    val: Expr,
}

#[derive(Clone, Debug)]
pub enum Statement {
    ShCall(Sh),
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
    statements: Vec<Statement>,
}
