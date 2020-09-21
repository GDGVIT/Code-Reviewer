use std::fmt;
use crate::grammar::Symbol;
use crate::lexical_analyser::token::{Token, TokenType};

#[derive(Clone, PartialEq)]
pub enum Atom {
    Var(Symbol),
    Tok(Token),
    TokType(TokenType),
    Epsilon
}

impl Atom {
    /// Returns value inside token if Atom variant is token; else returns None
    pub fn get_token_value(&self) -> Option<String> {
        if let Self::Tok(token) = self {
            return Some(token.value.clone());
        }

        None
    }

    pub fn from_token(tok: String) -> Atom {
        Atom::Tok(Token::from(tok))
    }
    
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Atom::Var(a) => write!(f, "{:?}", a),
            Atom::Tok(a) => write!(f, "{:?}", a),
            Atom::TokType(a) => write!(f, "{:?}", a),
            Atom::Epsilon => write!(f, "ε")
        }
    }
}

