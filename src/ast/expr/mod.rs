pub mod identifier;
#[cfg(test)]
mod tests;

use crate::lexer::Position;

pub use self::identifier::*;
pub mod call;
pub use self::call::*;
pub mod literal;
pub use self::literal::*;

use crate::ast::stmt::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Identifier(IdentifierExpr),
    Call(CallExpr),
    Literal(LiteralExpr),
    Block(Vec<Stmt>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub kind: ExprKind,
    pub pos: Position,
}

impl Expr {
    pub fn new(kind: ExprKind, pos: Position) -> Self {
        Self { kind, pos }
    }
}
