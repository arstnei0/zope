#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberChar {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl NumberChar {
    pub fn parse(char: &char) -> Option<NumberChar> {
        if *char == '1' {
            Some(NumberChar::One)
        } else if *char == '2' {
            Some(NumberChar::Two)
        } else if *char == '3' {
            Some(NumberChar::Three)
        } else if *char == '4' {
            Some(NumberChar::Four)
        } else if *char == '5' {
            Some(NumberChar::Five)
        } else if *char == '6' {
            Some(NumberChar::Six)
        } else if *char == '7' {
            Some(NumberChar::Seven)
        } else if *char == '8' {
            Some(NumberChar::Eight)
        } else if *char == '9' {
            Some(NumberChar::Nine)
        } else if *char == '0' {
            Some(NumberChar::Zero)
        } else {
            None
        }
    }

    pub fn int(&self) -> usize {
        match self {
            NumberChar::One => 1,
            NumberChar::Two => 2,
            NumberChar::Three => 3,
            NumberChar::Four => 4,
            NumberChar::Five => 5,
            NumberChar::Six => 6,
            NumberChar::Seven => 7,
            NumberChar::Eight => 8,
            NumberChar::Nine => 9,
            NumberChar::Zero => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_char() {
        assert_eq!(NumberChar::parse(&'1'), Some(NumberChar::One));
        assert_eq!(NumberChar::parse(&' '), None);
    }
}
