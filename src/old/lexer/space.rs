#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Space {
    Space,
    NewLine,
    Tab,
}

impl Space {
    pub fn parse(char: &char) -> Option<Space> {
        if *char == ' ' {
            Some(Space::Space)
        } else if *char == '\n' {
            Some(Space::NewLine)
        } else if *char == '\t' {
            Some(Space::Tab)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_space() {
        assert_eq!(Space$1, Some(Space::Space));
        assert_eq!(Space::parse(&'\n'), Some(Space::NewLine));
        assert_eq!(Space::parse(&'\t'), Some(Space::Tab));
    }
}
