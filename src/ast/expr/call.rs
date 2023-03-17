use super::Expr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub called: Box<Expr>,
    pub input: Box<Expr>,
}

impl Call {
    pub fn new(called: Expr, input: Expr) -> Self {
        Self {
            called: Box::new(called),
            input: Box::new(input),
        }
    }
}
