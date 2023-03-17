pub mod identifier;
#[cfg(test)]
mod tests;

use crate::lexer::Token;

pub use self::identifier::*;
pub mod call;
pub use self::call::*;
pub mod literal;
pub use self::literal::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Identifier(Identifier),
    Call(Call),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub kind: ExprKind,
    pub tokens: Vec<Token>,
}

impl Expr {
    pub fn new(kind: ExprKind, token: Vec<Token>) -> Self {
        Self {
            kind,
            tokens: token,
        }
    }
}
