pub trait IntoText {
    fn into_text(&mut self) -> Option<String>;
    fn get_path(&self) -> &str;
}

pub struct Lexer<'a> {
    pub content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    pub fn trim(&mut self) {
        while !self.content.is_empty() && self.content[0].is_ascii_whitespace() {
            self.content = &self.content[1..];
        }
    }

    pub fn cut(&mut self, n: usize) -> Option<&'a [char]> {
        if n >= self.content.len() {
            None
        } else {
            let result = &self.content[..n];
            self.content = &self.content[n..];
            Some(result)
        }
    }

    pub fn cut_at(&mut self, mut at: impl FnMut(&char) -> bool) -> Option<&'a [char]> {
        let mut n = 0;
        while n < self.content.len() && at(&self.content[n]) {
            n += 1;
        }
        self.cut(n)
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim();
        if self.content.is_empty() {
            return None;
        }
        if self.content[0].is_numeric() {
            let token = self.cut_at(|s| {
                s.is_numeric() && !s.is_ascii_punctuation() && !s.is_ascii_whitespace()
            });
            return token;
        }

        if self.content[0].is_alphanumeric() {
            let token = self.cut_at(|s| {
                s.is_alphanumeric() && !s.is_ascii_punctuation() && !s.is_ascii_whitespace()
            });
            return token;
        }
        if self.content[0].is_ascii_punctuation() {
            let token = &self.content[0..1];
            self.content = &self.content[1..];
            return Some(token);
        }
        None
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
