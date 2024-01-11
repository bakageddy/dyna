use poppler::PopplerDocument;
use std::path::PathBuf;
use crate::index::IntoText;

pub struct PDF {
    pub path: PathBuf,
}

impl PDF {
    pub fn get_all_text(&self) -> Result<String, ()> {
        let doc = PopplerDocument::new_from_file(&self.path, "").map_err(|e| {
            eprintln!(
                "Failed to load pdf file: {} because: {e}",
                self.path.display()
            );
        })?;
        let n = doc.get_n_pages();
        let mut content = String::new();
        for i in 0..n {
            if let Some(page) = doc.get_page(i) {
                if let Some(text) = page.get_text() {
                    content.push_str(text);
                    content.push('\n');
                }
            }
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
