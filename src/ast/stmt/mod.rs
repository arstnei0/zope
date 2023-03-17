use crate::lexer::*;

pub enum StmtKind {}

pub struct Stmt {
    pub kind: StmtKind,
    pub tokens: Vec<Token>,
}
