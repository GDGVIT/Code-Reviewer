use crate::lexical_analyser::token::tokentype::{TokenType};

#[derive(Debug)]
#[derive(Clone)]
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
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.value == other.value
    }
}


// // ** << >> <= >= !=