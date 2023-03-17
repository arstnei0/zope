use self::Bracket::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bracket {
    // (
    OpenParen,
    // )
    CloseParen,
    // {
    OpenCurly,
    // }
    CloseCurly,
    // [
    OpenSquare,
    // ]
    CloseSquare,
    // <
    OpenAngle,
    // >
    CloseAngle,
}

impl Bracket {
    pub fn parse(char: &char) -> Option<Bracket> {
        if *char == '(' {
            Some(OpenParen)
        } else if *char == ')' {
            Some(CloseParen)
        } else if *char == '{' {
            Some(OpenCurly)
        } else if *char == '}' {
            Some(CloseCurly)
        } else if *char == '[' {
            Some(OpenSquare)
        } else if *char == ']' {
            Some(CloseSquare)
        } else if *char == '<' {
            Some(OpenAngle)
        } else if *char == '>' {
            Some(CloseAngle)
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
        assert_eq!(Bracket::parse(&'('), Some(OpenParen));
        assert_eq!(Bracket::parse(&')'), Some(CloseParen));
        assert_eq!(Bracket::parse(&'{'), Some(OpenCurly));
        assert_eq!(Bracket::parse(&'}'), Some(CloseCurly));
        assert_eq!(Bracket::parse(&'['), Some(OpenSquare));
        assert_eq!(Bracket::parse(&']'), Some(CloseSquare));
        assert_eq!(Bracket::parse(&'<'), Some(OpenAngle));
        assert_eq!(Bracket::parse(&'>'), Some(CloseAngle));
        assert_eq!(Bracket::parse(&' '), None);
    }
}
