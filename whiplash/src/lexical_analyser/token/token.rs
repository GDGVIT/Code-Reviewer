use crate::lexical_analyser::token::tokentype::{TokenType};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Token<'a> {
    pub category: TokenType,
    pub value: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(category: TokenType, value: &'a str) -> Token {
        Token {
            category,
            value
        }
    }
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.value == other.value
    }
}


// // ** << >> <= >= !=