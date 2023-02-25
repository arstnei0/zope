use super::literal::*;
use crate::token::{bracket::*, punctuation::Punctuation, utils::*, *};

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
            let (func_token_stream, input_token_stream) =
                split_stream(in_paren, space_token_checker);
            let func = Expr::parse(func_token_stream);
            let input = Expr::parse(input_token_stream);

            if let Some(func) = func {
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
    Literal(Literal),
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
}
