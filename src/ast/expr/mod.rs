pub mod identifier;
#[cfg(test)]
mod tests;
use std::str::Chars;

use crate::lexer::bracket::Bracket;
use crate::lexer::*;

use self::identifier::*;
pub mod call;
use self::call::*;
pub mod literal;
use self::literal::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Identifier(Identifier),
    Call(Call),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    kind: ExprKind,
    token: Vec<Token>,
}

impl Expr {
    pub fn new(kind: ExprKind, token: Vec<Token>) -> Self {
        Self { kind, token }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn from_chars(chars: Chars<'a>) -> Self {
        Self {
            lexer: Lexer::new(chars),
        }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::from_chars(input.chars())
    }

    fn next_expr(lexer: &mut Lexer<'_>) -> Option<Expr> {
        let token = lexer.bump_next()?;
        let kind = &token.kind;

        match kind {
            TokenKind::Identifier(identifier) => {
                let identifier_expr = Expr::new(
                    ExprKind::Identifier(Identifier::new(identifier.to_owned())),
                    vec![token.clone()],
                );
                let call_input = {
                    if let Some(next_token) = lexer.peek_next() {
                        if let TokenKind::Bracket(Bracket::OpenParen) = next_token.kind {
                            let mut lexer = lexer.clone();
                            lexer.bump_next()?;

                            let expr = Self::next_expr(&mut lexer)?;

                            if let TokenKind::Bracket(Bracket::CloseParen) = lexer.bump_next()?.kind
                            {
                                Some(expr)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                if let Some(call_input) = call_input {
                    Some(Expr::new(
                        ExprKind::Call(Call::new(identifier_expr, call_input)),
                        vec![token],
                    ))
                } else {
                    Some(identifier_expr)
                }
            }
            _ => None,
        }
    }

    pub fn bump_next(&mut self) -> Option<Expr> {
        Self::next_expr(&mut self.lexer)
    }
}
