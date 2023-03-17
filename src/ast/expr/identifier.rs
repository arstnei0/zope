#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub ident: String,
}

impl Identifier {
    pub fn new(ident: String) -> Identifier {
        Identifier { ident }
    }
}
