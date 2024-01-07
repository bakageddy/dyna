use std::fs;
use std::collections::HashMap;

type Token = String;

#[derive(Debug)]
pub struct Tokenizer {
    pub name: String,
    pub content: Vec<u8>,
    pub tokens: Vec<Token>,
    cursor: usize,
    current: Option<u8>,
}

#[derive(Debug)]
pub struct DirIndex {
    pub dirname: String,
    pub indices: Vec<DocumentIndex>,
}

#[derive(Debug)]
pub struct DocumentIndex {
    pub filename: String,
    pub index: HashMap<String, i32>,
}


impl Tokenizer {
    pub fn new(file: &str) -> Self {
        let content: String;
        let current: Option<u8>;
        match fs::read_to_string(file) {
            Ok(t) => {
                content = t.trim_end().trim_start().to_string();
                current = content.as_bytes().get(0).and_then(|s| Some(*s));
            },
            Err(e) => {
                content = String::new();
                current = None;
                println!("File: {file} {e:#?}");
            }
        }
        Self {
            name: file.to_string(),
            tokens: Vec::new(),
            content: content.as_bytes().to_owned(),
            cursor: 0,
            current,
        }
    }

    pub fn peek(&self, ahead: Option<usize>) -> Option<u8> {
        let ahead = ahead.unwrap_or(1);
        if self.cursor + ahead > self.content.len() {
            return None;
        } else {
            return self.content.get(self.cursor + ahead).and_then(|x| Some(*x));
        }
    }

    pub fn consume(&mut self) {
        self.current = self.peek(None);
        self.cursor += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if c.is_ascii_whitespace() {
                self.consume();
            } else {
                return;
            }
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tok = String::new();
        self.skip_whitespace();
        while let Some(c) = self.current {

            if let Some(next) = self.peek(None) {
                if next.is_ascii_whitespace() {
                    tok.push(c as char);
                    self.consume();
                    return Some(tok);
                }
                if next.is_ascii_punctuation() || c.is_ascii_punctuation() {
                    tok.push(c as char);
                    self.consume();
                    return Some(tok);
                }
            }

            tok.push(c as char);
            self.consume();
        }
        None
    }
}
