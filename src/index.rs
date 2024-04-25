use std::{
    collections::HashMap,
    fs,
    io::{self, BufWriter, ErrorKind},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    books::BookFile,
    docx::DocxFile,
    lexer::{IntoText, Lexer},
    stemmer::{self},
    text::TextFile,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileIndex {
    name: String,
    index: HashMap<String, usize>,
}

impl FileIndex {
    pub fn new(file: &mut impl IntoText) -> Option<Self> {
        if let Some(contents) = file.into_text() {
            let mut index = HashMap::new();
            let contents = contents.chars().collect::<Vec<_>>();
            for i in Lexer::new(&contents) {
                let key = i.iter().collect();
                let stemmed_word = stemmer::stem(key);
                index
                    .entry(stemmed_word)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
            Some(Self {
                name: file.get_path().to_string(),
                index,
            })
        } else {
            None
        }
    }

    pub fn get_tf(&self, term: &str) -> Option<usize> {
        self.index.get(term).map(|s| *s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirIndex<'a> {
    name: &'a str,
    files: Vec<FileIndex>,
    df: HashMap<String, f64>,
}

impl<'a> DirIndex<'a> {
    pub fn new(path: &'a str) -> Option<Self> {
        let mut p = PathBuf::new();
        p.push(path);
        if let Some(paths) = get_all_paths(&p) {
            let mut document_freq = HashMap::new();
            let mut index_files = Vec::new();
            for i in paths {
                if let Some(etx) = i.extension() {
                    let index: FileIndex;

                    if etx.eq("pdf") || etx.eq("epub") {
                        let mut file = BookFile::new(&i);
                        if let Some(idx) = FileIndex::new(&mut file) {
                            index = idx;
                        } else {
                            continue;
                        }
                    } else if etx.eq("docx") {
                        let mut file = DocxFile::new(&i);
                        if let Some(idx) = FileIndex::new(&mut file) {
                            index = idx;
                        } else {
                            continue;
                        }
                    } else {
                        let mut file = TextFile::new(&i.display().to_string());
                        if let Some(idx) = FileIndex::new(&mut file) {
                            index = idx;
                        } else {
                            continue;
                        }
                    }

                    for (token, tf) in &index.index {
                        document_freq
                            .entry(token.clone())
                            .and_modify(|v| *v += *tf)
                            .or_insert(*tf);
                    }
                    index_files.push(index);
                }
            }
            let n = index_files.len();
            let df = document_freq
                .into_iter()
                .map(|(k, v)| {
                    let score = (n as f64) / v as f64;
                    (k, score.log10())
                })
                .collect();
            Some(Self {
                name: path,
                df,
                files: index_files,
            })
        } else {
            None
        }
    }

    pub fn save_to_file<P>(&self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let f = fs::File::create(path)?;
        let writer = BufWriter::new(f);
        if let Ok(_) = serde_json::to_writer_pretty(writer, self) {
            Ok(())
        } else {
            Err(std::io::Error::new(
                ErrorKind::WriteZero,
                "Failed to write to index",
            ))
        }
    }

    pub fn search_term(&self, term: &str) -> HashMap<String, f64> {
        let mut scores = HashMap::new();
        for index in &self.files {
            for word in term.to_lowercase().split_whitespace() {
                let word = stemmer::stem(word.to_owned());
                if let Some(tf) = index.get_tf(&word) {
                    if let Some(idf) = self.df.get(&word) {
                        let score = (*idf) * (tf as f64);
                        scores
                            .entry(index.name.clone())
                            .and_modify(|v| *v += score)
                            .or_insert(score);
                    }
                }
            }
        }
        scores
    }
}

pub fn get_all_paths(dir: &PathBuf) -> Option<Vec<PathBuf>> {
    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for etr in entries {
            if let Ok(etr) = etr {
                let entry = etr.path();
                if entry.is_dir() {
                    if let Some(sub_paths) = get_all_paths(&entry) {
                        paths.extend(sub_paths);
                    } else {
                        continue;
                    }
                } else if entry.is_file() {
                    paths.push(entry);
                } else if entry.is_symlink() {
                    // TODO: Symlinks
                    continue;
                }
            } else {
                continue;
            }
        }
        return Some(paths);
    } else {
        return None;
    }
}
