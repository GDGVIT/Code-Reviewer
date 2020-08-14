use crate::tokenizer::tokenizer::Tokenizer;
use std::fs;

mod tokenizer;
mod ast;

fn main() {
    let contents = fs::read_to_string("../test1.txt").unwrap();

    let tokenizer = Tokenizer::new();
    println!("{:#?}", tokenizer.tokenize_line(&contents[..]));
}
