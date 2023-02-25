#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Punctuation {
    Comma,
    FullStop,
    Semicolon,
}

impl Punctuation {
    pub fn parse(char: char) -> Option<Punctuation> {
        if char == ',' {
            Some(Punctuation::Comma)
        } else if char == '.' {
            Some(Punctuation::FullStop)
        } else if char == ';' {
            Some(Punctuation::Semicolon)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_punctuation() {
        assert_eq!(Punctuation::parse(','), Some(Punctuation::Comma));
        assert_eq!(Punctuation::parse('.'), Some(Punctuation::FullStop));
        assert_eq!(Punctuation::parse(';'), Some(Punctuation::Semicolon));
        assert_eq!(Punctuation::parse(' '), None);
    }
}
