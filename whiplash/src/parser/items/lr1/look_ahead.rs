use std::fmt;
use std::hash::Hash;
use crate::grammar::Symbol;
use crate::lexical_analyser::token::tokentype::TokenType;

#[derive(Hash)]
pub enum Atom {
    Var(Symbol),
    Tok(TokenType)
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Atom::Var(a) => write!(f, "{:?}", a),
            Atom::Tok(a) => write!(f, "{:?}", a)
        }
    }
}

#[derive(Hash)]
pub struct Atoms {
    vals: Vec<Atom>
}

impl fmt::Debug for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self.vals.iter().fold(
                String::new(),
                |acc, atom| acc + "/" + &format!("{:?}", atom)[..]
            )
        )
    }
}
