#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Separator {
    SingleQuote,
    DoubleQuote,
    Backtick,
}

impl Separator {
    pub fn parse(char: char) -> Option<Separator> {
        if char == '\'' {
            Some(Separator::SingleQuote)
        } else if char == '"' {
            Some(Separator::DoubleQuote)
        } else if char == '`' {
            Some(Separator::Backtick)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_keyword() {
        assert_eq!(Separator::parse('\''), Some(Separator::SingleQuote));
        assert_eq!(Separator::parse('"'), Some(Separator::DoubleQuote));
        assert_eq!(Separator::parse('`'), Some(Separator::Backtick));
        assert_eq!(Separator::parse(' '), None);
    }
}
