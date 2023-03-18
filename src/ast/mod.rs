pub mod expr;
pub mod stmt;
#[cfg(test)]
mod tests;

use std::str::Chars;

pub use self::expr::*;
pub use self::stmt::*;
use crate::lexer::*;

macro_rules! expr {
    ($kind:ident, $content:expr, $pos:expr) => {
        Expr::new(ExprKind::$kind($content), $pos)
    };
}

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn from_lexer(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn from_chars(chars: Chars<'a>) -> Self {
        Self::from_lexer(Lexer::new(chars))
    }

    pub fn new(input: &'a str) -> Self {
        Self::from_chars(input.chars())
    }

    fn parse_stmt_inner(lexer_: &mut Lexer) -> Option<Stmt> {
        let mut lexer = lexer_.clone();
        let first = lexer.bump()?;
        let stmt_matched = match first.kind {
            TokenKind::Keyword(Keyword::Let) => {
                lexer.ignore_spaces();
                let identifier_token = lexer.bump()?;
                if let TokenKind::Identifier(identifier) = identifier_token.kind {
                    lexer.ignore_spaces();
                    let equal_token = lexer.bump()?;
                    if let TokenKind::Operator(Operator::Equal) = equal_token.kind {
                        lexer.ignore_spaces();
                        let value_expr = Self::parse_expr_inner(&mut lexer)?;
                        Some(Stmt::new(
                            StmtKind::Let(LetStmt::new(identifier, value_expr)),
                            Position::new(first.pos.start, 0),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(stmt_) = stmt_matched {
            let semicolon = lexer.ignore_semicolon();
            let mut stmt = stmt_;
            if let Some(semicolon) = semicolon {
                stmt.pos.end = semicolon.pos.end;
            }
            Some(stmt)
        } else {
            let parsed_expr = Self::parse_expr_inner(lexer_);
            if let Some(expr) = parsed_expr {
                let start = expr.pos.start;
                let mut end = expr.pos.end;

                // Detect semicolon
                let semicolon = lexer.ignore_semicolon();
                if let Some(semicolon) = semicolon {
                    end = semicolon.pos.end;
                }
                Some(Stmt::new(StmtKind::Expr(expr), Position::new(start, end)))
            } else {
                None
            }
        }
    }

    pub fn parse_expr_call(lexer_: &mut Lexer) -> Option<Expr> {
        let lexer__ = &mut (lexer_.clone());
        let called = Self::parse_expr_not_call(lexer__);
        let lexer = &mut (lexer__.clone());

        if let Some(called) = called {
            let open_bracket = lexer.bump();
            if let Some(open_bracket) = open_bracket {
                if let TokenKind::Bracket(Bracket::OpenParen) = open_bracket.kind {
                    let call_input = {
                        let expr = Self::parse_expr_inner(lexer)?;

                        Some(expr)
                    };

                    if let Some(call_input) = call_input {
                        let close_bracket = lexer.bump()?;
                        if let TokenKind::Bracket(Bracket::CloseParen) = close_bracket.kind {
                            lexer_.sync(lexer.clone());
                            let call = CallExpr::new(called.to_owned(), call_input);
                            return Some(Expr::new(
                                ExprKind::Call(call),
                                Position::new(called.pos.start, close_bracket.pos.end),
                            ));
                        }
                    }
                }
            }
            lexer_.sync(lexer__.clone());
            Some(called)
        } else {
            None
        }
    }

    fn parse_expr_not_call(lexer_: &mut Lexer) -> Option<Expr> {
        let mut lexer = lexer_.clone();
        let token = lexer.bump()?;
        let kind = &token.kind;
        let pos = &token.pos;

        let result = match kind {
            // Identifier
            TokenKind::Identifier(identifier) => Some(Expr::new(
                ExprKind::Identifier(IdentifierExpr::new(identifier.to_owned())),
                Position::new(pos.start, pos.end),
            )),

            // Number
            TokenKind::NumberChar(number_char) => {
                let mut number_string = vec![number_char.clone()];
                let mut end = token.pos.end;

                loop {
                    let mut lexer_ = lexer.clone();
                    let token = lexer_.bump();
                    if let Some(token) = token {
                        if let TokenKind::NumberChar(current_number_char) = token.kind {
                            number_string.push(current_number_char);
                            end = token.pos.end;
                            lexer.sync(lexer_);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let numbers: Vec<usize> = number_string.iter().map(|s| s.int()).collect();
                let mut number = 0u32;
                for n in numbers.iter() {
                    number *= 10;
                    number += *n as u32;
                }

                Some(Expr::new(
                    ExprKind::Literal(LiteralExpr::Number(number)),
                    Position::new(pos.start, end),
                ))
            }

            // Bool
            TokenKind::Keyword(Keyword::True) => Some(Expr::new(
                ExprKind::Literal(LiteralExpr::Bool(true)),
                Position::new(pos.start, pos.end),
            )),
            TokenKind::Keyword(Keyword::False) => Some(Expr::new(
                ExprKind::Literal(LiteralExpr::Bool(false)),
                Position::new(pos.start, pos.end),
            )),

            // String
            TokenKind::Separation(separation) => Some(expr!(
                Literal,
                LiteralExpr::String(separation.separated.clone()),
                Position::new(token.pos.start, token.pos.end)
            )),

            _ => None,
        };

        if let Some(_) = result {
            lexer_.sync(lexer);
        }

        result
    }

    fn parse_expr_inner(lexer: &mut Lexer) -> Option<Expr> {
        Self::parse_expr_call(lexer)
    }

    pub fn parse_expr(&mut self) -> Option<Expr> {
        Self::parse_expr_inner(&mut self.lexer)
    }

    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        Self::parse_stmt_inner(&mut self.lexer)
    }

    pub fn reload_lexer(&mut self, lexer: Lexer<'a>) {
        self.lexer = lexer
    }

    pub fn reload_chars(&mut self, chars: Chars<'a>) {
        self.reload_lexer(Lexer::new(chars))
    }

    pub fn reload(&mut self, input: &'a str) {
        self.reload_chars(input.chars())
    }
}
