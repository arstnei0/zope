#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Separator {
    #[default]
    SingleQuote,
    DoubleQuote,
}

impl Separator {
    pub fn parse(char: &char) -> Option<Self> {
        match *char {
            '"' => Some(Self::DoubleQuote),
            '\'' => Some(Self::SingleQuote),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Separation {
    pub separator: Separator,
    pub separated: String,
}

impl Separation {
    pub fn new(separator: Separator, separated: String) -> Self {
        Self {
            separator,
            separated,
        }
    }
}
