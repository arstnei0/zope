#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    Let,
    Fn,
    State,
    If,
}

impl Keyword {
    pub fn parse(ident: &String) -> Option<Keyword> {
        match ident.as_str() {
            "let" => Some(Keyword::Let),
            "fn" => Some(Keyword::Fn),
            "if" => Some(Keyword::If),
            "state" => Some(Keyword::State),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_keyword() {
        assert_eq!(Keyword::parse(&"let".into()), Some(Keyword::Let));
        assert_eq!(Keyword::parse(&"fn".into()), Some(Keyword::Fn));
        assert_eq!(Keyword::parse(&"state".into()), Some(Keyword::State));
        assert_eq!(Keyword::parse(&"hello_world".into()), None);
    }
}
