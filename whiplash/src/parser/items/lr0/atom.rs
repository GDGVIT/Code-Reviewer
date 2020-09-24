use std::fmt;
use std::hash::Hash;

use crate::grammar::Symbol;
use crate::lexical_analyser::token::tokentype::TokenType;
use crate::grammar::production;

#[derive(Hash)]
pub enum Atom {
    Val(production::Atom),
    Dot
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Val(a) => write!(f, "{:?}", a),
            Self::Dot => write!(f, ".")
        }
    }
}

#[derive(Hash)]
pub struct Atoms {
    vals: Vec<Atom>
}

impl Atoms {
    pub fn from(v: Vec<Atom>) -> Atoms {
        Atoms {
            vals: v
        }
    }
}

impl fmt::Debug for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self.vals.iter().fold(
                String::new(), 
                |acc, atom| acc + &format!("{:?}", &atom)[..] + " "
            )
        )
    }
}