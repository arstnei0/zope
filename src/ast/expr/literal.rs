#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralExpr {
    Number(u32),
    Bool(bool),
    String(String),
}
