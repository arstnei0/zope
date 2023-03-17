use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    pub position: u32,
}

impl<'a> Cursor<'a> {
    pub fn new(chars: Chars<'a>) -> Cursor<'a> {
        Cursor { chars, position: 0 }
    }

    #[inline]
    pub fn peek(&self) -> Chars {
        self.chars.clone()
    }

    #[inline]
    pub fn first(&self) -> Option<char> {
        self.peek().next()
    }

    #[inline]
    pub fn second(&self) -> Option<char> {
        let mut chars = self.peek();
        chars.next();
        chars.next()
    }

    #[inline]
    pub fn bump(&mut self) -> Option<char> {
        self.position += 1;
        self.chars.next()
    }
}
