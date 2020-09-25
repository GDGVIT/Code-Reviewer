use std::fmt;
use crate::grammar::Symbol;
use crate::lexical_analyser::token::{Token, TokenType};

#[derive(Clone, PartialEq, Hash)]
pub enum Terminal {
    Tok(Token),
    Epsilon
}

#[derive(Clone, PartialEq, Hash)]
pub enum NonTerminal {
    Sym(Symbol),
    TokType(TokenType)
}

#[derive(Clone, PartialEq, Hash)]
pub enum Atom {
    Term(Terminal),
    NonTerm(NonTerminal)
}

impl Atom {
    /// Returns value inside token if Atom variant is token; else returns None
    pub fn get_token_value(&self) -> Option<String> {
        if let Self::Term(Terminal::Tok(token)) = self {
            return Some(token.value.clone());
        }

        None
    }

    pub fn from_token(tok: String) -> Atom {
        Atom::Term(
            Terminal::Tok(
                Token::from(tok)
            )
        )
    }

    pub fn from_epsilon() -> Atom {
        Atom::Term(
            Terminal::Epsilon
        )
    }
    
}

impl fmt::Debug for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Tok(token) => write!(f, "{:?}", token),
            Self::Epsilon => write!(f, "ε") 
        }
    }
}

impl fmt::Debug for NonTerminal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::TokType(t) => write!(f, "{:?}", t),
            Self::Sym(s) => write!(f, "{:?}", s)
        }
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Term(t) => write!(f, "{:?}", t),
            Self::NonTerm(nt) => write!(f, "{:?}", nt)
        }
    }
}
