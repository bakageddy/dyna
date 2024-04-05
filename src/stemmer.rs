extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};

pub fn stem(word: String) -> String {
    let transformer = Stemmer::create(Algorithm::English);
    transformer.stem(word.to_lowercase().as_str()).to_string()
}
