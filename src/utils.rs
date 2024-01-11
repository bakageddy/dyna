use crate::{
    index::{DirIndex, DocumentIndex, Tokenizer, TextFile},
    pdf::PDF,
};
use std::{
    collections::HashMap,
    fs,
    io::{self, BufReader, BufWriter, Error, ErrorKind},
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    // Directory to index
    #[arg(long)]
    pub dir: Option<String>,
    // Name of the file to save
    #[arg(long)]
    pub index: Option<String>,
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

pub fn index_file(filename: PathBuf) -> io::Result<DocumentIndex> {
    let mut tf = HashMap::new();
    let mut index = HashMap::new();
    let lexer;
    let path = filename.clone();

    if filename.extension().unwrap_or("".as_ref()).eq("pdf") {
        let mut pdf = PDF { path: filename };
        lexer = Tokenizer::file(&mut pdf).ok();
    } else if filename.extension().unwrap_or("".as_ref()).eq("txt"){
        let mut txt = TextFile { path: filename };
        lexer = Tokenizer::file(&mut txt).ok();
    } else {
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    if let Some(lexer) = lexer {
        let tokens: Vec<String> = lexer.into_iter().collect();
        let n = tokens.len();

        for token in tokens {
            index.entry(token).and_modify(|c| *c += 1).or_insert(1);
        }

        for (token, count) in &index {
            tf.insert(token.clone(), (*count as f32) / (n as f32));
        }

        Ok(DocumentIndex {
            filename: String::from(path.to_string_lossy()),
            tf,
            index,
        })
    } else {
        Err(Error::from(ErrorKind::InvalidInput))
    }
}

pub fn index_dir(dir_name: &str) -> io::Result<DirIndex> {
    let dir = Path::new(dir_name).to_path_buf();
    let mut indices = Vec::new();
    if dir.exists() && dir.is_dir() {
        let paths = get_all_file_paths(dir)?;
        for i in paths {
            let file_index = index_file(i);
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

pub fn search_term(term: String, index: &DirIndex) -> HashMap<&String, f32> {
    let mut occurences = HashMap::new();
    for entry in &index.indices {
        for i in term.split_whitespace() {
            if entry.index.contains_key(i) {
                let tf = entry.tf[i];
                let df = index.df[i];
                let score = tf * df;
                occurences
                    .entry(&entry.filename)
                    .and_modify(|v| *v += score)
                    .or_insert(score);
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
                // Skip indexing hidden files
                if entry.path().starts_with(".") {
                    continue;
                }

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
