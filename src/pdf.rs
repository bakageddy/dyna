use std::path::Path;
use mupdf::{document::Document, page::Page};

use crate::lexer::IntoText;

pub struct PdfFile<P> {
    pub filename: P, 
}

impl<P: AsRef<Path>> PdfFile<P> {
    pub fn new(filename: P) -> Self {
        Self { filename }
    }
}

impl<P: AsRef<Path>> IntoText for PdfFile<P> {
    fn into_text(&mut self) -> Option<String> {
        let mut content = String::new();
        if let Ok(doc) =  Document::open(self.get_path()) {
            if let Ok(pages) = doc.into_iter().collect::<Result<Vec<Page>, _>>() {
                for i in pages {
                    if let Ok(page_content) = i.to_text() {
                        content.push_str(&page_content);
                    }
                }
                Some(content.to_lowercase())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_path(&self) -> &str {
        self.filename.as_ref().to_str().unwrap()
    }
}
