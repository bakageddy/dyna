mod index;
mod utils;

use clap::Parser;

use crate::utils::*;

fn main() {
    let args = Args::parse();
    if let Ok(index) = utils::index_dir(args.index.as_str()) {
        match save_to_file(&index, "./test.index") {
            Ok(_) => {
                println!("Saved to file!");
                return;
            }
            Err(e) => {
                eprintln!("Failed to save to file: {e}");
                return;
            }
        }
    } 
}
