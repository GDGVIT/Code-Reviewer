use crate::tokenizer::tokenizer::Tokenizer;
use std::fs;
use std::io::{self, prelude::*};

mod tokenizer;
mod ast;

fn main() -> io::Result<()> {
    let file = fs::File::open("../test1.txt")?;
    let reader = io::BufReader::new(file);
    let tokenizer = Tokenizer::new();

    for line in reader.lines() {
        println!("{:#?}", tokenizer.tokenize_line(&line?[..]));
    }

    Ok(())
}
