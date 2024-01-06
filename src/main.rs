mod index;
mod utils;
use std::process::exit;

use crate::{utils::*, index::Tokenizer};

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    if args.len() == 1 {
        usage();
    } else {
        if args[1] == "--help" {
            usage();
            exit(0);
        }
    }

    for i in Tokenizer::new("./test/xenos.txt") {
        println!("{i}")
    }

    // let index = utils::index_document("./test");
    // println!("{index:?}");
}
