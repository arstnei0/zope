#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentifierExpr {
    pub ident: String,
}

impl IdentifierExpr {
    pub fn new(ident: String) -> IdentifierExpr {
        IdentifierExpr { ident }
    }
}
