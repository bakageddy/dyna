mod index;
mod stemmer;
mod utils;

use clap::Parser;

use crate::utils::*;

fn main() {
    let args = Args::parse();
    if let (Some(dir), Some(save_loc)) = (args.index, &args.save) {
        if let Ok(index) = index_dir(dir.as_str()) {
            match save_to_file(&index, save_loc.as_str()) {
                Ok(_) => {
                    println!("Saved to file!");
                }
                Err(e) => {
                    eprintln!("Failed to save to file: {e}");
                }
            }
        }
    }

    if let (Some(term), Some(index_location)) = (args.search, &args.save) {
        if let Some(index) = load_index(index_location.as_str()) {
            let result = search_term(term, &index);
            println!("{result:?}");
        } else {
            println!("Failed to load index: {index_location}");
        }
    }
}
