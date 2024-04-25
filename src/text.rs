use crate::lexer::IntoText;
use std::{
    fs,
    io::{BufReader, Read},
};

pub struct TextFile {
    pub filename: String,
}

impl TextFile {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }
}

impl IntoText for TextFile {
    fn into_text(&mut self) -> Option<String> {
        if let Ok(handle) = fs::File::open(self.get_path()) {
            let mut buf = BufReader::new(handle);
            let mut content = String::new();
            if let Ok(_) = buf.read_to_string(&mut content) {
                Some(content.to_lowercase())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_path(&self) -> &str {
        &self.filename
    }
}
