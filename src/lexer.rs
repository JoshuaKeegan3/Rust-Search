pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        let mut n = 0;
        while n < self.content.len() && self.content[n].is_alphanumeric() {
            n += 1;
        }
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        return Some(token);
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && !self.content[0].is_alphanumeric() {
            self.content = &self.content[1..];
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
