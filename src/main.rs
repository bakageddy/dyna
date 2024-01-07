mod index;
mod utils;

use crate::utils::*;

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    if args.len() == 1 {
        usage();
    } else {
        if args[1] == "--help" {
            usage();
            return;
        } else if args[1] == "--index" {
            if let Ok(index) = utils::index_dir(args[2].as_str()) {
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
            } else {
                eprintln!("Failed to index dir!");
                return;
            }
        } else {
            usage();
            return;
        }
    }
}
