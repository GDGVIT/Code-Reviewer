use std::fmt;
use crate::parser::node::nodetype::NodeType;
use crate::lexical_analyser::token::tokentype::TokenType;

pub enum Atom {
    Var(NodeType),
    Tok(TokenType),
    Dot
}

impl fmt::Debug for Atom {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match Atom {
            Var(a) => write!(f, "{:?}", a),
            Tok(a) => write!(f, "{:?}", a),
            Dot => write!(f, ".")
        }
    }
}

pub type Atoms = Vec<Atom>;

impl fmt::Debug for Vec<Atom> {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            &self.iter().fold(
                String::new(), 
                |acc, &atom| acc + dbg!(&atom) + " "
            )
        )
    }
}