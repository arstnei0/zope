#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {
    Let,
    Fn,
    State,
}

impl Keyword {
    pub fn parse(ident: String) -> Option<Keyword> {
        if ident == "let" {
            Some(Keyword::Let)
        } else if ident == "fn" {
            Some(Keyword::Fn)
        } else if ident == "state" {
            Some(Keyword::State)
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
        assert_eq!(Keyword::parse("let".into()), Some(Keyword::Let));
        assert_eq!(Keyword::parse("fn".into()), Some(Keyword::Fn));
        assert_eq!(Keyword::parse("state".into()), Some(Keyword::State));
        assert_eq!(Keyword::parse("hello_world".into()), None);
    }
}
