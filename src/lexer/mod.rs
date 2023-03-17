pub mod bracket;
pub mod cursor;
pub mod keyword;
pub mod number;
pub mod operator;
pub mod position;
pub mod punctuation;
pub mod space;
#[cfg(test)]
mod tests;
use std::str::Chars;

use cursor::*;

pub use self::{
    bracket::*, keyword::*, number::*, operator::*, position::*, punctuation::*, space::*,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Bracket(Bracket),
    Space(Space),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Identifier(String),
    Operator(Operator),
    NumberChar(NumberChar),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
}

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            cursor: Cursor::new(chars),
        }
    }

    fn parse(cursor: &mut Cursor<'_>) -> Option<Token> {
        let mut pos = Position {
            start: cursor.position,
            end: cursor.position,
        };
        let char = &cursor.bump()?;

        if let Some(kind) = Self::parse_char(char) {
            Some(Token { pos, kind })
        } else {
            let mut identifier = String::new();
            let mut curr_char = *char;
            loop {
                identifier.push(curr_char);
                if let Some(char) = cursor.first() {
                    curr_char = char;
                    if let Some(_) = Self::parse_char(&curr_char) {
                        break;
                    } else {
                        cursor.bump();
                        pos.end += 1;
                    }
                } else {
                    break;
                }
            }

            if let Some(keyword) = Keyword::parse(&identifier) {
                Some(Token {
                    pos,
                    kind: TokenKind::Keyword(keyword),
                })
            } else {
                Some(Token {
                    pos,
                    kind: TokenKind::Identifier(identifier),
                })
            }
        }
    }

    pub fn bump(&mut self) -> Option<Token> {
        Self::parse(&mut self.cursor)
    }

    pub fn ignore(&mut self) {
        self.cursor.bump();
    }

    pub fn ignore_n(&mut self, n: usize) {
        for _ in 0..n {
            self.ignore();
        }
    }

    pub fn sync(&mut self, another: Self) {
        self.cursor = another.cursor;
    }

    pub fn parse_char(char: &char) -> Option<TokenKind> {
        if let Some(result) = Bracket::parse(char) {
            Some(TokenKind::Bracket(result))
        } else if let Some(result) = Space::parse(char) {
            Some(TokenKind::Space(result))
        } else if let Some(result) = Punctuation::parse(char) {
            Some(TokenKind::Punctuation(result))
        } else if let Some(result) = NumberChar::parse(char) {
            Some(TokenKind::NumberChar(result))
        } else if let Some(result) = Operator::parse(char) {
            Some(TokenKind::Operator(result))
        } else {
            None
        }
    }
}
