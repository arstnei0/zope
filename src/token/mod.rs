pub mod bracket;
pub mod keyword;
pub mod punctuation;
pub mod separator;
pub mod space;

use self::bracket::*;
use self::keyword::*;
use self::punctuation::*;
use self::separator::*;
use self::space::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenT {
    Bracket(Bracket),
    Space(Space),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Ident(String),
    Separator(Separator),
    Separated(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenPos {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub pos: TokenPos,
    pub t: TokenT,
}

pub type TokenStream = Vec<Token>;

pub fn tokenize(code: String) -> TokenStream {
    let mut token_stream = Vec::new();
    let mut code_clone = code.clone();
    code_clone.push(' ');

    let chars = code_clone.chars();

    let mut in_ident = false;
    let mut curr_ident_start = 0usize;
    let mut curr_ident: String = "".into();

    let mut in_separator = false;
    let mut curr_separated_start = 0usize;
    let mut curr_separated: String = "".into();
    let mut opened_separator = Separator::SingleQuote;

    for (i, char) in chars.enumerate() {
        let bracket_res = Bracket::parse(char);
        let space_res = Space::parse(char);
        let punctuation_res = Punctuation::parse(char);
        let separator_res = Separator::parse(char);
        let is_ident = {
            if let Some(_) = bracket_res {
                false
            } else if let Some(_) = space_res {
                false
            } else if let Some(_) = punctuation_res {
                false
            } else if let Some(_) = separator_res {
                false
            } else {
                true
            }
        };

        if in_separator {
            let closed = if let Some(separator) = separator_res.clone() {
                if separator == opened_separator {
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if closed {
                token_stream.push(Token {
                    pos: TokenPos {
                        start: curr_separated_start,
                        end: i - 2,
                    },
                    t: TokenT::Separated(curr_separated.clone()),
                });
                let close_separator_start = i - 1;
                token_stream.push(Token {
                    pos: TokenPos {
                        start: close_separator_start,
                        end: close_separator_start,
                    },
                    t: TokenT::Separator(opened_separator.clone()),
                });
                in_separator = false;
            } else {
                curr_separated.push(char);
                continue;
            }
        } else if in_ident {
            if is_ident {
                curr_ident.push(char);
                continue;
            } else {
                if let Some(keyword) = Keyword::parse(curr_ident.clone()) {
                    token_stream.push(Token {
                        pos: TokenPos {
                            start: curr_ident_start,
                            end: i - 1,
                        },
                        t: TokenT::Keyword(keyword),
                    });
                    in_ident = false;
                } else {
                    token_stream.push(Token {
                        pos: TokenPos {
                            start: curr_ident_start,
                            end: i - 1,
                        },
                        t: TokenT::Ident(curr_ident.clone()),
                    });
                    in_ident = false;
                }
            }
        }

        let this_pos = TokenPos { start: i, end: i };

        if let Some(bracket) = bracket_res {
            token_stream.push(Token {
                pos: this_pos,
                t: TokenT::Bracket(bracket),
            });
        } else if let Some(space) = space_res {
            token_stream.push(Token {
                pos: this_pos,
                t: TokenT::Space(space),
            });
        } else if let Some(punctuation) = punctuation_res {
            token_stream.push(Token {
                pos: this_pos,
                t: TokenT::Punctuation(punctuation),
            });
        } else if let Some(separator) = separator_res {
            token_stream.push(Token {
                pos: this_pos,
                t: TokenT::Separator(separator.clone()),
            });
            curr_separated_start = i;
            curr_separated = "".into();
            opened_separator = separator;
            in_separator = true;
        } else {
            curr_ident = char.to_string();
            curr_ident_start = i;
            in_ident = true;
        }
    }

    token_stream.pop();

    token_stream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_brackets() {
        let res = tokenize("()".to_string());
        assert_eq!(
            res.get(0).unwrap().clone(),
            Token {
                pos: TokenPos { start: 0, end: 0 },
                t: TokenT::Bracket(Bracket::parse('(').unwrap())
            }
        );
        assert_eq!(
            res.get(1).unwrap().clone(),
            Token {
                pos: TokenPos { start: 1, end: 1 },
                t: TokenT::Bracket(Bracket::parse(')').unwrap())
            }
        );
    }

    #[test]
    fn tokenize_space() {
        let res = tokenize(" \n\t".to_string());
        assert_eq!(
            res.get(0).unwrap().clone().t,
            TokenT::Space(Space::parse(' ').unwrap())
        );
        assert_eq!(
            res.get(1).unwrap().clone().t,
            TokenT::Space(Space::parse('\n').unwrap())
        );
        assert_eq!(
            res.get(2).unwrap().clone().t,
            TokenT::Space(Space::parse('\t').unwrap())
        );
    }

    #[test]
    fn tokenize_punctuation() {
        let res = tokenize(".".to_string());
        assert_eq!(
            res.get(0).unwrap().clone().t,
            TokenT::Punctuation(Punctuation::FullStop),
        );
    }

    #[test]
    fn tokenize_ident() {
        let res = tokenize("hello_world".to_string());
        assert_eq!(
            res.get(0).unwrap().clone().t,
            TokenT::Ident("hello_world".into()),
        );
    }

    #[test]
    fn tokenize_keyword() {
        let res = tokenize("let fn".to_string());
        assert_eq!(res.get(0).unwrap().clone().t, TokenT::Keyword(Keyword::Let),);
        assert_eq!(res.get(2).unwrap().clone().t, TokenT::Keyword(Keyword::Fn),);
    }

    #[test]
    fn tokenize_separator() {
        let res = tokenize("'hello'".to_string());
        assert_eq!(
            res.get(0).unwrap().clone().t,
            TokenT::Separator(Separator::SingleQuote),
        );
        assert_eq!(
            res.get(1).unwrap().clone().t,
            TokenT::Separated("hello".into()),
        );
        assert_eq!(
            res.get(2).unwrap().clone().t,
            TokenT::Separator(Separator::SingleQuote),
        );
    }
}
