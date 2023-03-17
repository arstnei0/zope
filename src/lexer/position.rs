#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub start: u32,
    pub end: u32,
}

impl Position {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn get(&self, string: &str) -> Option<String> {
        let mut new_str = "".to_string();

        let mut chars = string.chars();
        let mut i = 0;
        loop {
            if i >= self.start {
                break;
            }
            chars.next();
            i += 1;
        }

        loop {
            new_str.push(chars.next()?);
            if i >= self.end {
                break;
            }
            i += 1;
        }

        Some(new_str)
    }
}
