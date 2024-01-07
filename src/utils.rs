use crate::index::{DirIndex, DocumentIndex, Tokenizer};
use std::{
    collections::HashMap,
    io::{self, BufWriter},
    path::{Path, PathBuf}, fs,
};

pub fn usage() {
    println!("      --help                Print this information");
    println!("      --index [dir]         Index the provided dir");
}

pub fn save_to_file(index: &DirIndex, file: &str) -> io::Result<()> {
    let f = fs::File::create(file)?;
    let w = BufWriter::new(f);
    serde_json::to_writer(w, index)?;
    Ok(())
}

pub fn index_file(filename: &str) -> DocumentIndex {
    let mut index = HashMap::new();
    let lexer = Tokenizer::new(filename);
    for token in lexer {
        match index.get_mut(&token) {
            Some(count) => {
                *count += 1;
            }
            None => {
                index.insert(token, 1);
            }
        }
    }
    DocumentIndex {
        filename: String::from(filename),
        index,
    }
}

pub fn index_dir(dir_name: &str) -> io::Result<DirIndex> {
    let dir = Path::new(dir_name).to_path_buf();
    let mut indices = Vec::new();
    if dir.exists() && dir.is_dir() {
        let paths = get_all_file_paths(dir)?;
        for i in paths {
            let file_index = index_file(i.to_str().unwrap_or(""));
            indices.push(file_index);
        }
    }

    Ok(DirIndex {
        dirname: dir_name.to_string(),
        indices,
    })
}

pub fn get_all_file_paths(dir: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        match entry {
            Ok(entry) => {
                if entry.path().is_dir() {
                    paths.extend(get_all_file_paths(entry.path())?);
                } else if entry.path().is_file() {
                    paths.push(entry.path());
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    return Ok(paths);
}
