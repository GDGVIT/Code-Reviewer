use crate::lexical_analyser::token::tokentype::{TokenType};
use crate::lexical_analyser::tokenizer::Tokenizer;

#[derive(Clone, Debug, Hash)]
pub struct Token {
    pub category: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(category: TokenType, value: String) -> Token {
        Token {
            category,
            value
        }
    }

    pub fn from(s: String) -> Token {
        let tokenizer = Tokenizer::new();
        tokenizer.identify_token(s)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.value == other.value
    }
}


// // ** << >> <= >= !=