#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Number(u32),
    Bool(bool),
}
