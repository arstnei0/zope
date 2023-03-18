use crate::ast::expr::*;
use crate::lexer::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStmt {
    pub identifier: String,
    pub value: Expr,
}

impl LetStmt {
    pub fn new(identifier: String, value: Expr) -> LetStmt {
        Self { identifier, value }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StmtKind {
    Expr(Expr),
    Let(LetStmt),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub pos: Position,
}

impl Stmt {
    pub fn new(kind: StmtKind, pos: Position) -> Stmt {
        Self { kind, pos }
    }
}
