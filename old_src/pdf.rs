use std::path::PathBuf;
use crate::index::IntoText;

use mupdf::{document::Document, page::Page};

pub struct PDF {
    pub path: PathBuf,
}

impl PDF {
    pub fn get_all_text(&self) -> Result<String, ()> {
        let mut content = String::new();
        if let Ok(doc) =  Document::open(&self.path.to_string_lossy()) {
            if let Ok(pages) = doc.into_iter().collect::<Result<Vec<Page>, _>>() {
                for i in pages {
                    if let Ok(page_content) = i.to_text() {
                        content.push_str(&page_content);
                    }
                }
            }
        } else {
            return Err(());
        }
        Ok(content)
    }
}

impl IntoText for PDF {
    fn into_text(&mut self) -> Option<String> {
        self.get_all_text().ok()
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
