mod index;
mod utils;
use std::{collections::HashMap, process::exit};

use crate::utils::*;

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

    let mut index: HashMap<String, i32> = HashMap::new();
    let t = index::Tokenizer::new("books/glAttachShader.xhtml");
    for token in t {
        match index.get_mut(&token) {
            Some(count) => {
                *count += 1;
            },
            None => {
                index.insert(token, 1);
            }
        }
    }
    println!("{index:#?}")
}
