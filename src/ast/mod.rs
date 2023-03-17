pub mod expr;
pub mod stmt;
#[cfg(test)]
mod tests;

use std::str::Chars;

pub use self::expr::*;
pub use self::stmt::*;
use crate::lexer::*;

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

    pub fn parse_call(lexer_: &mut Lexer) -> Option<Expr> {
        let lexer = &mut (lexer_.clone());
        let called = Self::parse_no_call(lexer);
        let mut tokens = Vec::new();
        let open_bracket = lexer.bump()?;

        if let TokenKind::Bracket(Bracket::OpenParen) = open_bracket.kind {
            if let Some(called) = called {
                let call_input = {
                    if let Some(next_token) = lexer.bump() {
                        tokens.push(next_token.to_owned());
                        if let TokenKind::Bracket(Bracket::OpenParen) = next_token.kind {
                            tokens.push(next_token);

                            let expr = Self::next_expr(lexer)?;

                            if let TokenKind::Bracket(Bracket::CloseParen) = lexer.bump()?.kind {
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
                    if let Some(close_bracket) = lexer.bump() {
                        if let TokenKind::Bracket(Bracket::CloseParen) = close_bracket.kind {
                            lexer_.sync(lexer.clone());
                            let call = Call::new(called.to_owned(), call_input);
                            return Some(Expr::new(ExprKind::Call(call), tokens));
                        }
                    }
                }
            }
        }
        None
    }

    fn parse_no_call(lexer_: &mut Lexer) -> Option<Expr> {
        let mut lexer = lexer_.clone();
        let token = lexer.bump()?;
        let kind = &token.kind;

        match kind {
            TokenKind::Identifier(identifier) => {
                lexer_.sync(lexer);
                Some(Expr::new(
                    ExprKind::Identifier(Identifier::new(identifier.to_owned())),
                    vec![token.to_owned()],
                ))
            }
            _ => None,
        }
    }

    fn next_expr(lexer: &mut Lexer<'_>) -> Option<Expr> {
        if let Some(call_expr) = Self::parse_call(lexer) {
            Some(call_expr)
        } else {
            if let Some(expr) = Self::parse_no_call(lexer) {
                Some(expr)
            } else {
                None
            }
        }
    }

    pub fn bump_next(&mut self) -> Option<Expr> {
        Self::next_expr(&mut self.lexer)
    }
}
