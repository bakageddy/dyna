use crate::{index::{DirIndex, DocumentIndex, Tokenizer}, stemmer::stem_this};
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    // Directory to index
    #[arg(long)]
    pub index: Option<String>,
    // Name of the file to save
    #[arg(long)]
    pub save: Option<String>,
    // Term(s) to search
    #[arg(long)]
    pub search: Option<String>,

    // #[arg(long)]
    // pub stem: Option<bool>,
}

pub fn save_to_file(index: &DirIndex, file: &str) -> io::Result<()> {
    let f = fs::File::create(file)?;
    let w = BufWriter::new(f);
    serde_json::to_writer(w, index)?;
    Ok(())
}

pub fn load_index(index_location: &str) -> Option<DirIndex> {
    if let Ok(f) = fs::File::open(index_location) {
        let buf = BufReader::new(f);
        if let Ok(result) = serde_json::from_reader(buf) {
            Some(result)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn index_file(filename: &str) -> io::Result<DocumentIndex> {
    let mut index = HashMap::new();
    let lexer = Tokenizer::new(filename)?;
    for token in lexer {
        match index.get_mut(&token) {
            Some(count) => {
                *count += 1;
            }
            None => {
                index.insert(token.to_lowercase(), 1);
            }
        }
    }
    Ok(DocumentIndex {
        filename: String::from(filename),
        index,
    })
}

pub fn index_dir(dir_name: &str) -> io::Result<DirIndex> {
    let dir = Path::new(dir_name).to_path_buf();
    let mut indices = Vec::new();
    if dir.exists() && dir.is_dir() {
        let paths = get_all_file_paths(dir)?;
        for i in paths {
            let file_index = index_file(i.to_str().unwrap_or(""));
            if let Ok(file_index) = file_index {
                indices.push(file_index);
            }
        }
    }

    Ok(DirIndex::new(
        dir_name.to_string(),
        indices,
        std::time::SystemTime::now(),
    ))
}

pub fn search_term(term: String, index: &DirIndex) -> HashSet<&String> {
    let mut occurences = HashSet::new();
    for entry in &index.indices {
        for i in term.split_whitespace() {
            if entry.index.contains_key(&stem_this(i.to_lowercase())) {
                occurences.insert(&entry.filename);
            }
        }
    }
    occurences
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
