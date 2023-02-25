#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Number(i128),
    Boolean(bool),
    String(String),
}
