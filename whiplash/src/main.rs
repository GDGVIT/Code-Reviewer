extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::lexical_analyser::tokenizer::tokenizer::Tokenizer;
use std::fs;
use std::io::{self, prelude::*};

mod lexical_analyser;
mod parser;
mod grammar;

fn main() -> io::Result<()> {
    let file = fs::File::open("../test1.py")?;
    let reader = io::BufReader::new(file);
    let tokenizer = Tokenizer::new();

    for line in reader.lines() {
        println!("{:?}", tokenizer.tokenize_line(&line?[..]));
    }

    Ok(())
}
