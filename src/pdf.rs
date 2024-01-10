use poppler::PopplerDocument;
use std::path::PathBuf;

pub fn get_all_text(file_name: PathBuf) -> Result<String, ()> {
    let doc = PopplerDocument::new_from_file(&file_name, "").map_err(|e| {
        eprintln!(
            "Failed to load pdf file: {} because: {e}",
            file_name.display()
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

