use crate::token::{bracket::*, punctuation::Punctuation, separator::Separator, utils::*, *};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralExpr {
    Number(i128),
    Boolean(bool),
    String(String),
}

impl LiteralExpr {
    pub fn parse(token_stream: TokenStream) -> ParseExprResult {
        let mut token_stream = skip_spaces(token_stream);
        let first = token_stream.first();

        if let Some(first) = first {
            if let TokenT::Separator(Separator::DoubleQuote) = first.t {
                token_stream.remove(0);
                let string = token_stream.swap_remove(0);

                if let TokenT::Separated(string) = string.t {
                    token_stream.remove(0);
                    Some(ParseExprResultSome {
                        expr: Expr::Literal(LiteralExpr::String(string)),
                        rest: token_stream,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseExprResultSome {
    pub expr: Expr,
    pub rest: TokenStream,
}
pub type ParseExprResult = Option<ParseExprResultSome>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentExpr {
    pub ident: String,
}

impl IdentExpr {
    pub fn parse(token_stream: TokenStream) -> ParseExprResult {
        let mut token_stream = token_stream.clone();
        let first = token_stream.first();
        if let Some(first) = first {
            if let TokenT::Ident(ident) = first.t.clone() {
                token_stream.remove(0);
                Some(ParseExprResultSome {
                    expr: Expr::Ident(IdentExpr { ident: ident }),
                    rest: token_stream,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallExpr {
    pub func: Box<Expr>,
    pub input: Box<Expr>,
}

impl CallExpr {
    fn parse(token_stream: TokenStream) -> ParseExprResult {
        let with_paren = deliminated(
            skip_spaces(token_stream.clone()),
            TokenT::Bracket(Bracket::Open(BracketT::Paren)),
            TokenT::Bracket(Bracket::Close(BracketT::Paren)),
        );

        if let Some((in_paren, rest)) = with_paren {
            let func = Expr::parse(in_paren);

            if let Some(func) = func {
                let input = Expr::parse(skip_spaces(func.rest));
                if let Some(input) = input {
                    Some(ParseExprResultSome {
                        expr: Expr::Call(CallExpr {
                            func: Box::new(func.expr),
                            input: Box::new(input.expr),
                        }),
                        rest: rest,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccesserExpr {
    pub accessed: Box<Expr>,
    pub accessing: String,
}

impl AccesserExpr {
    pub fn parse(token_stream: TokenStream) -> ParseExprResult {
        let accessed = Expr::parse_not_accesser(token_stream);
        if let Some(accessed) = accessed {
            let mut rest = accessed.rest;
            let should_be_fullstop = rest.first();
            if let Some(should_be_fullstop) = should_be_fullstop {
                if let TokenT::Punctuation(Punctuation::FullStop) = should_be_fullstop.t {
                    rest.remove(0);
                    let accessing_ident = rest.first();
                    if let Some(accessing_ident) = accessing_ident {
                        if let TokenT::Ident(ident) = accessing_ident.t.clone() {
                            rest.remove(0);
                            Some(ParseExprResultSome {
                                expr: Expr::Accesser(AccesserExpr {
                                    accessed: Box::new(accessed.expr),
                                    accessing: ident,
                                }),
                                rest: rest,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(LiteralExpr),
    Call(CallExpr),
    Accesser(AccesserExpr),
    Ident(IdentExpr),
}

impl Expr {
    pub fn parse_not_accesser(token_stream: TokenStream) -> ParseExprResult {
        if let Some(call_expr) = CallExpr::parse(token_stream.clone()) {
            Some(call_expr)
        } else if let Some(ident_expr) = IdentExpr::parse(token_stream.clone()) {
            Some(ident_expr)
        } else if let Some(literal_expr) = LiteralExpr::parse(token_stream.clone()) {
            Some(literal_expr)
        } else {
            None
        }
    }

    pub fn parse(token_stream: TokenStream) -> ParseExprResult {
        if let Some(accesser_expr) = AccesserExpr::parse(token_stream.clone()) {
            Some(accesser_expr)
        } else if let Some(expr) = Expr::parse_not_accesser(token_stream.clone()) {
            Some(expr)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ident() {
        let token_stream = tokenize("hello".into());
        assert_eq!(
            IdentExpr::parse(token_stream),
            Some(ParseExprResultSome {
                expr: Expr::Ident(IdentExpr {
                    ident: "hello".into()
                }),
                rest: vec![]
            })
        )
    }

    #[test]
    fn parse_accesser() {
        let token_stream = tokenize("hello.world".into());
        assert_eq!(
            AccesserExpr::parse(token_stream),
            Some(ParseExprResultSome {
                expr: Expr::Accesser(AccesserExpr {
                    accessed: Box::new(Expr::Ident(IdentExpr {
                        ident: "hello".into()
                    })),
                    accessing: "world".into()
                }),
                rest: vec![]
            })
        );
    }

    #[test]
    fn parse_call() {
        let token_stream = tokenize("(hello world)".into());
        assert_eq!(
            CallExpr::parse(token_stream),
            Some(ParseExprResultSome {
                expr: Expr::Call(CallExpr {
                    func: Box::new(Expr::Ident(IdentExpr {
                        ident: "hello".into()
                    })),
                    input: Box::new(Expr::Ident(IdentExpr {
                        ident: "world".into()
                    })),
                }),
                rest: vec![]
            })
        );
    }

    #[test]
    fn parse_literal() {
        let token_stream = tokenize("\"Hello world\"".into());
        assert_eq!(
            LiteralExpr::parse(token_stream),
            Some(ParseExprResultSome {
                expr: Expr::Literal(LiteralExpr::String("Hello world".into())),
                rest: vec![]
            })
        );
    }
}
