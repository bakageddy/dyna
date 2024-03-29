use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{fs, io};

pub trait IntoText {
    fn into_text(&mut self) -> Option<String>;
    fn get_path(&self) -> PathBuf;
}

type Token = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokenizer {
    pub name: String,
    pub content: Vec<u8>,
    cursor: usize,
    current: Option<u8>,
}

#[derive(Debug)]
pub struct TextFile {
    pub path: PathBuf,
}

impl IntoText for TextFile {
    fn into_text(&mut self) -> Option<String> {
        if let Ok(f) = fs::File::open(self.get_path()) {
            let mut buf = BufReader::new(f);
            let mut content = String::new();
            let _ = buf.read_to_string(&mut content);
            return Some(content);
        }
        None
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirIndex {
    pub dirname: String,
    pub indices: Vec<DocumentIndex>,
    pub df: HashMap<String, f32>,
    index_time: std::time::SystemTime,
}

impl DirIndex {
    pub fn new(
        dirname: String,
        indices: Vec<DocumentIndex>,
        index_time: std::time::SystemTime,
    ) -> Self {
        let mut document_token_freq = HashMap::new();
        let n = indices.len();
        for doc in &indices {
            for (token, count) in &doc.index {
                document_token_freq
                    .entry(token)
                    .and_modify(|c| *c += count)
                    .or_insert(1);
            }
        }

        let df = document_token_freq
            .into_iter()
            .map(|(k, v)| {
                let score = n as f32 / (1 + v) as f32;
                (k.clone(), score.log10() + 1f32)
            })
            .collect();

        Self {
            dirname,
            indices,
            index_time,
            df,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentIndex {
    pub filename: String,
    pub index: HashMap<String, i32>,
    pub tf: HashMap<String, f32>,
}

impl Tokenizer {
    pub fn new(file: &str) -> io::Result<Self> {
        let f = fs::File::open(file)?;
        let mut content = Vec::new();
        let _ = BufReader::new(f).read_to_end(&mut content);
        let current = content.get(0).and_then(|s| Some(*s));

        Ok(Self {
            name: file.to_string(),
            content,
            current,
            cursor: 0,
        })
    }

    pub fn from(something: &mut impl IntoText) -> io::Result<Self> {
        if let Some(content) = something.into_text() {
            let content = content.into_bytes();
            let current = content.get(0).and_then(|s| Some(*s));
            let name = something.get_path().display().to_string();
            Ok(Self {
                name,
                current,
                content,
                cursor: 0,
            })
        } else {
            return Err(std::io::Error::new(io::ErrorKind::Other, "No Content"));
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
