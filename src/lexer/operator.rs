#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Equal,
    Slash,
}

impl Operator {
    pub fn parse(char: &char) -> Option<Operator> {
        if *char == '=' {
            Some(Operator::Equal)
        } else if *char == '/' {
            Some(Operator::Slash)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bracket() {
        assert_eq!(Operator::parse(&'='), Some(Operator::Equal));
        assert_eq!(Operator::parse(&'/'), Some(Operator::Slash));
        assert_eq!(Operator::parse(&' '), None);
    }
}
