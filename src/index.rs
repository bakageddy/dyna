use std::fs;
use std::collections::HashMap;

type Token = String;

#[derive(Debug)]
pub struct Tokenizer {
    pub name: String,
    pub content: Vec<u8>,
    pub tokens: Vec<Token>,
    cursor: usize,
    current: u8,
}

pub struct DirIndex {
    pub dirname: String,
    pub indices: Vec<DocumentIndex>,
}

pub struct DocumentIndex {
    pub filename: String,
    pub index: HashMap<String, i32>,
}


impl Tokenizer {
    pub fn new(file: &str) -> Self {
        let content = fs::read_to_string(file).ok().unwrap_or(String::new());
        let current = content.as_bytes()[0];
        Self {
            name: file.to_string(),
            tokens: Vec::new(),
            content: content.as_bytes().to_owned(),
            cursor: 0,
            current,
        }
    }

    pub fn peek(&self) -> u8 {
        if self.cursor + 1 < self.content.len() {
            return self.content[self.cursor + 1];
        }
        return 0
    }

    pub fn consume(&mut self) {
        if self.cursor + 1 < self.content.len() {
            self.cursor += 1;
            self.current = self.content[self.cursor]
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.current.is_ascii_whitespace() {
            self.consume();
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tok = String::new();
        while self.peek() != 0 {
            self.skip_whitespace();

            if self.peek().is_ascii_punctuation() || self.current.is_ascii_punctuation() {
                tok.push(self.current as char);
                self.consume();
                return Some(tok);
            }

            if self.peek().is_ascii_whitespace() {
                tok.push(self.current as char);
                self.consume();
                return Some(tok);
            }
            tok.push(self.current as char);
            self.consume();
        }
        None
    }
}
