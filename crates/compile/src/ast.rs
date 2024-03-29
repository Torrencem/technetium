//! Abstract Syntax Tree types representing parsed Technetium programs

use codespan::Span;

use crate::script;
use lexer::error::*;
use lexer::Lexer;

/// Common functionality for AST expression nodes
pub trait AstExpr {
    /// Increase the indices of the Spans in this AST node by a
    /// given offset. This is useful when this AST node is part
    /// of a format string, and the indices need to be offset to
    /// be accurate for the entire file.
    fn offset_spans(&mut self, offset: usize);
}

/// Any kind of literal written in code. Each variant represents
/// a different kind of literal
#[derive(Clone, Debug)]
pub enum Literal {
    Unit(Span),
    Integer(i64, Span),
    Float(f64, Span),
    Str(String, Span),
    Char(char, Span),
    Bool(bool, Span),
    FormatString(FormatString),
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::Unit(s) => *s,
            Literal::Integer(_, s) => *s,
            Literal::Float(_, s) => *s,
            Literal::Str(_, s) => *s,
            Literal::Char(_, s) => *s,
            Literal::Bool(_, s) => *s,
            Literal::FormatString(s) => s.span,
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        match self {
            Literal::Unit(s) => s,
            Literal::Integer(_, s) => s,
            Literal::Float(_, s) => s,
            Literal::Str(_, s) => s,
            Literal::Char(_, s) => s,
            Literal::Bool(_, s) => s,
            Literal::FormatString(s) => &mut s.span,
        }
    }
}

impl AstExpr for Literal {
    fn offset_spans(&mut self, offset: usize) {
        if let Literal::FormatString(fs) = self {
            fs.offset_spans(offset);
        } else {
            let l = self.span().start();
            let r = self.span().end();
            *self.span_mut() = Span::new(
                u32::from(l) + (offset as u32),
                u32::from(r) + (offset as u32),
            );
        }
    }
}

/// A list of expressions surrounded by square brackets
/// in code: ``[1, 2, "hello"]``
#[derive(Clone, Debug)]
pub struct ListLiteral {
    pub span: Span,
    pub values: Vec<Expr>,
}

impl ListLiteral {
    pub fn new(values: Vec<Expr>, l: usize, r: usize) -> Self {
        ListLiteral {
            span: Span::new(l as u32, r as u32),
            values,
        }
    }
}

impl AstExpr for ListLiteral {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        for val in self.values.iter_mut() {
            val.offset_spans(offset);
        }
    }
}

/// A list of expressions surrounded by parenthesis
/// in code: ``(1, 2, "hello")``
#[derive(Clone, Debug)]
pub struct TupleLiteral {
    pub span: Span,
    pub values: Vec<Expr>,
}

impl TupleLiteral {
    pub fn new(values: Vec<Expr>, l: usize, r: usize) -> Self {
        TupleLiteral {
            span: Span::new(l as u32, r as u32),
            values,
        }
    }
}

impl AstExpr for TupleLiteral {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        for val in self.values.iter_mut() {
            val.offset_spans(offset);
        }
    }
}

/// A list of expressions surrounded by brackets
/// in code: ``{1, 2, "hello"}``
#[derive(Clone, Debug)]
pub struct SetLiteral {
    pub span: Span,
    pub values: Vec<Expr>,
}

impl SetLiteral {
    pub fn new(values: Vec<Expr>, l: usize, r: usize) -> Self {
        SetLiteral {
            span: Span::new(l as u32, r as u32),
            values,
        }
    }
}

impl AstExpr for SetLiteral {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        for val in self.values.iter_mut() {
            val.offset_spans(offset);
        }
    }
}

/// A literal dictionary given in code: ``{"a": true, "b": 123}``
#[derive(Clone, Debug)]
pub struct DictLiteral {
    pub span: Span,
    pub values: Vec<(Expr, Expr)>,
}

impl DictLiteral {
    pub fn new(values: Vec<(Expr, Expr)>, l: usize, r: usize) -> Self {
        DictLiteral {
            span: Span::new(l as u32, r as u32),
            values,
        }
    }
}

impl AstExpr for DictLiteral {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        for (vala, valb) in self.values.iter_mut() {
            vala.offset_spans(offset);
            valb.offset_spans(offset);
        }
    }
}

/// A function call, not attached to a particular parent
/// (i.e., not a method call): ``print("123")``
#[derive(Clone, Debug)]
pub struct FuncCall {
    pub span: Span,
    pub func: Box<Expr>,
    pub arguments: Vec<Expr>,
}

impl FuncCall {
    pub fn new(func: Expr, arguments: Vec<Expr>, l: usize, r: usize) -> Self {
        FuncCall {
            span: Span::new(l as u32, r as u32),
            func: Box::new(func),
            arguments,
        }
    }
}

impl AstExpr for FuncCall {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        self.func.offset_spans(offset);
        for arg in self.arguments.iter_mut() {
            arg.offset_spans(offset);
        }
    }
}

/// An attribute expression without a function call
/// (i.e., not a method call): ``person.name``
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

impl AstExpr for AttrLookup {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        self.attribute.offset_spans(offset);
        self.parent.offset_spans(offset);
    }
}

/// A method call on an expression: ``[1, 2].length()``
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
            fname,
            arguments,
        }
    }
}

impl AstExpr for MethodCall {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        self.fname.offset_spans(offset);
        self.parent.offset_spans(offset);
        for arg in self.arguments.iter_mut() {
            arg.offset_spans(offset);
        }
    }
}

/// An expression indexed with square brackets:
/// ``my_list[i + 2]``
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

impl AstExpr for IndexedExpr {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        self.index.offset_spans(offset);
        self.parent.offset_spans(offset);
    }
}

/// An expression with a slice attached to it:
/// ``hello[1:3]`` or ``hello[10::-1]``
#[derive(Clone, Debug)]
pub struct SlicedExpr {
    pub span: Span,
    pub parent: Box<Expr>,
    pub start: Option<Box<Expr>>,
    pub end: Option<Box<Expr>>,
    pub step: Option<Box<Expr>>,
}

impl SlicedExpr {
    pub fn new(
        parent: Expr,
        start: Option<Expr>,
        end: Option<Expr>,
        step: Option<Expr>,
        l: usize,
        r: usize,
    ) -> Self {
        SlicedExpr {
            span: Span::new(l as u32, r as u32),
            parent: Box::new(parent),
            start: start.map(|val| Box::new(val)),
            end: end.map(|val| Box::new(val)),
            step: step.map(|val| Box::new(val)),
        }
    }
}

impl AstExpr for SlicedExpr {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        self.parent.offset_spans(offset);
        for one in self.start.iter_mut() {
            one.offset_spans(offset);
        }
        for one in self.end.iter_mut() {
            one.offset_spans(offset);
        }
        for one in self.step.iter_mut() {
            one.offset_spans(offset);
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Variable(Identifier),
    Literal(Literal),
    ListLiteral(ListLiteral),
    TupleLiteral(TupleLiteral),
    SetLiteral(SetLiteral),
    DictLiteral(DictLiteral),
    MethodCall(MethodCall),
    FuncCall(FuncCall),
    AttrLookup(AttrLookup),
    IndexedExpr(IndexedExpr),
    SlicedExpr(SlicedExpr),
    PostPreOp(PostPreOp),
    AnonFuncDefinition(AnonFuncDefinition),
    /// An unreachable expression state, used to finish attempting to parse an AST (read
    /// <http://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html>
    Error,
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Variable(i) => i.span,
            Expr::Literal(l) => l.span(),
            Expr::ListLiteral(l) => l.span,
            Expr::TupleLiteral(t) => t.span,
            Expr::SetLiteral(l) => l.span,
            Expr::DictLiteral(d) => d.span,
            Expr::MethodCall(m) => m.span,
            Expr::FuncCall(f) => f.span,
            Expr::AttrLookup(a) => a.span,
            Expr::IndexedExpr(e) => e.span,
            Expr::SlicedExpr(e) => e.span,
            Expr::PostPreOp(e) => e.span,
            Expr::AnonFuncDefinition(e) => e.span,
            Expr::Error => unreachable!(),
        }
    }
}

impl AstExpr for Expr {
    fn offset_spans(&mut self, offset: usize) {
        match self {
            Expr::Variable(i) => i.offset_spans(offset),
            Expr::Literal(l) => l.offset_spans(offset),
            Expr::ListLiteral(l) => l.offset_spans(offset),
            Expr::TupleLiteral(t) => t.offset_spans(offset),
            Expr::SetLiteral(s) => s.offset_spans(offset),
            Expr::DictLiteral(d) => d.offset_spans(offset),
            Expr::MethodCall(m) => m.offset_spans(offset),
            Expr::FuncCall(f) => f.offset_spans(offset),
            Expr::AttrLookup(a) => a.offset_spans(offset),
            Expr::IndexedExpr(e) => e.offset_spans(offset),
            Expr::SlicedExpr(e) => e.offset_spans(offset),
            Expr::PostPreOp(e) => e.offset_spans(offset),
            Expr::AnonFuncDefinition(e) => e.offset_spans(offset),
            Expr::Error => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PPOVariant {
    PostIncrement,
    PreIncrement,
    PostDecrement,
    PreDecrement,
}

/// Some kind of post/pre-increment/decrement attached
/// to a expr: ``x++`` or ``--a[0]``
#[derive(Clone, Debug)]
pub struct PostPreOp {
    pub span: Span,
    pub variant: PPOVariant,
    pub val: AssignmentLHS,
}

impl PostPreOp {
    pub fn new_post_inc(val: AssignmentLHS, l: usize, r: usize) -> Self {
        PostPreOp {
            span: Span::new(l as u32, r as u32),
            variant: PPOVariant::PostIncrement,
            val,
        }
    }

    pub fn new_pre_inc(val: AssignmentLHS, l: usize, r: usize) -> Self {
        PostPreOp {
            span: Span::new(l as u32, r as u32),
            variant: PPOVariant::PreIncrement,
            val,
        }
    }

    pub fn new_post_dec(val: AssignmentLHS, l: usize, r: usize) -> Self {
        PostPreOp {
            span: Span::new(l as u32, r as u32),
            variant: PPOVariant::PostDecrement,
            val,
        }
    }

    pub fn new_pre_dec(val: AssignmentLHS, l: usize, r: usize) -> Self {
        PostPreOp {
            span: Span::new(l as u32, r as u32),
            variant: PPOVariant::PreDecrement,
            val,
        }
    }
}

impl AstExpr for PostPreOp {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
    }
}

#[derive(Clone, Debug)]
pub struct AnonFuncDefinition {
    pub span: Span,
    pub args: Vec<Identifier>,
    pub body: StatementList,
}

impl AnonFuncDefinition {
    pub fn new(args: Vec<Identifier>, body: StatementList, l: usize, r: usize) -> Self {
        AnonFuncDefinition {
            span: Span::new(l as u32, r as u32),
            args,
            body,
        }
    }
}

impl AstExpr for AnonFuncDefinition {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
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
            binding,
            iter,
            body,
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
            cond,
            body,
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

/// A possible additional clause on an if statement
#[derive(Clone, Debug)]
pub enum IfTail {
    ElseIf(Box<IfStatement>),
    Else(StatementList),
}

impl IfStatement {
    pub fn new(
        condition: Expr,
        then_body: StatementList,
        tail: Option<IfTail>,
        l: usize,
        r: usize,
    ) -> Self {
        IfStatement {
            span: Span::new(l as u32, r as u32),
            condition,
            then_body,
            tail,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CaseOf {
    pub span: Span,
    pub condition: Expr,
    pub cases: Vec<(Expr, StatementList)>,
    pub default: Option<StatementList>,
}

impl CaseOf {
    pub fn new(condition: Expr, cases: Vec<(Expr, StatementList)>, default: Option<StatementList>, l: usize, r: usize) -> Self {
        CaseOf {
            span: Span::new(l as u32, r as u32),
            condition,
            default,
            cases,
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
    pub fn new(
        name: Identifier,
        args: Vec<Identifier>,
        body: StatementList,
        l: usize,
        r: usize,
    ) -> Self {
        FuncDefinition {
            span: Span::new(l as u32, r as u32),
            name,
            args,
            body,
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
            ret,
        }
    }
}

/// A line beginning in ``$`` to make a shell statement
#[derive(Clone, Debug)]
pub struct ShStatement {
    pub span: Span,
    pub inner: FormatString,
}

/// A string preceded by ``~``, used to substitute variables into a string
#[derive(Clone, Debug)]
pub struct FormatString {
    pub span: Span,
    pub val: String,
    pub substitutions: Vec<Expr>,
}

impl FormatString {
    pub fn new(
        val: String,
        substitutions: Vec<(usize, String)>,
        l: usize,
        r: usize,
    ) -> Result<Self, MiscParseError> {
        let mut subs = vec![];
        for s in substitutions.iter() {
            let lexer = Lexer::new(s.1.as_ref());
            // Recursive multiple errors are ignored, so errors is not used
            let mut errors = vec![];
            let mut e = script::ExprParser::new().parse(&mut errors, lexer);
            offset_parse_result_error_spans(&mut e, s.0 + 1);
            let mut e = e?;
            if errors.len() > 0 {
                let mut primary_error = errors[0].error.clone();
                offset_parse_error_spans(&mut primary_error, s.0 + 1);
                return Err(primary_error.into());
            }
            e.offset_spans(s.0 + 1);
            subs.push(e);
        }
        Ok(FormatString {
            span: Span::new(l as u32, r as u32),
            val,
            substitutions: subs,
        })
    }
}

impl AstExpr for FormatString {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
        for sub in self.substitutions.iter_mut() {
            sub.offset_spans(offset);
        }
    }
}

/// A type of expression that can be on the left of an assignment:
/// either a variable, an identifier, or an indexed expr
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

    pub fn from_expr(val: Expr) -> Option<Self> {
        match val {
            Expr::Variable(id) => Some(AssignmentLHS::Identifier(id)),
            Expr::AttrLookup(al) => Some(AssignmentLHS::AttrLookup(al)),
            Expr::IndexedExpr(ie) => Some(AssignmentLHS::Indexed(ie)),
            _ => None,
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
            lhs,
            val,
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
    /// A dummy value used for parser recovery
    Error,
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
            Statement::Error => unreachable!(),
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
            statements,
        }
    }
}

/// A name given as part of an expression: ``print``
#[derive(Clone, Debug)]
pub struct Identifier {
    pub span: Span,
    pub name: String,
}

impl Identifier {
    pub fn new(name: String, l: usize, r: usize) -> Self {
        Identifier {
            span: Span::new(l as u32, r as u32),
            name,
        }
    }

    pub fn new_expr(name: String, l: usize, r: usize) -> Expr {
        Expr::Variable(Identifier {
            span: Span::new(l as u32, r as u32),
            name,
        })
    }
}

impl AstExpr for Identifier {
    fn offset_spans(&mut self, offset: usize) {
        let l = self.span.start();
        let r = self.span.end();
        self.span = Span::new(
            u32::from(l) + (offset as u32),
            u32::from(r) + (offset as u32),
        );
    }
}
