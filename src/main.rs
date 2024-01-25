mod index;
mod lexer;
mod text;
mod pdf;

use std::{fs, io::{BufReader, Read}};

use anyhow;
use index::DirIndex;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub dir: Option<String>,

    #[arg(long)]
    pub index: Option<String>,

    #[arg(long)]
    pub search: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let (Some(save_loc), Some(dir)) = (&args.index, args.dir) {
        if let Some(index) = DirIndex::new(&dir) {
            if let Ok(_) = index.save_to_file(&save_loc) {
                println!("Saved to location: {save_loc}");
            } else {
                println!("Failed to save index to location: {save_loc}");
            }
        } else {
            println!("Failed to index directory: {dir}");
        }
    } else if let (Some(index_location), Some(search_terms)) = (&args.index, args.search) {
        if let Ok(handle) = fs::File::open(&index_location) {
            let mut content = String::new();
            let mut rdr = BufReader::new(handle);
            let _ = rdr.read_to_string(&mut content);
            if let Ok(dir_index) = serde_json::from_str::<DirIndex>(&content) {
                let result = dir_index.search_term(&search_terms);
                println!("{result:#?}");
            } else {
                println!("Failed to load data from {index_location}");
            }
        } else {
            println!("Failed to open file: {index_location}");
        }
    } else {
        println!("Consider using --help");
    }
    Ok(())
}
