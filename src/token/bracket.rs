#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BracketT {
    Paren,
    Curly,
    Square,
    Angle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bracket {
    Open(BracketT),
    Close(BracketT),
}

impl Bracket {
    pub fn parse(char: char) -> Option<Bracket> {
        if char == '(' {
            Some(Bracket::Open(BracketT::Paren))
        } else if char == ')' {
            Some(Bracket::Close(BracketT::Paren))
        } else if char == '{' {
            Some(Bracket::Open(BracketT::Curly))
        } else if char == '}' {
            Some(Bracket::Close(BracketT::Curly))
        } else if char == '[' {
            Some(Bracket::Open(BracketT::Square))
        } else if char == ']' {
            Some(Bracket::Close(BracketT::Square))
        } else if char == '<' {
            Some(Bracket::Open(BracketT::Angle))
        } else if char == '>' {
            Some(Bracket::Close(BracketT::Angle))
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
        assert_eq!(Bracket::parse('('), Some(Bracket::Open(BracketT::Paren)));
        assert_eq!(Bracket::parse(')'), Some(Bracket::Close(BracketT::Paren)));
        assert_eq!(Bracket::parse('{'), Some(Bracket::Open(BracketT::Curly)));
        assert_eq!(Bracket::parse('}'), Some(Bracket::Close(BracketT::Curly)));
        assert_eq!(Bracket::parse('['), Some(Bracket::Open(BracketT::Square)));
        assert_eq!(Bracket::parse(']'), Some(Bracket::Close(BracketT::Square)));
        assert_eq!(Bracket::parse('<'), Some(Bracket::Open(BracketT::Angle)));
        assert_eq!(Bracket::parse('>'), Some(Bracket::Close(BracketT::Angle)));
        assert_eq!(Bracket::parse(' '), None);
    }
}
