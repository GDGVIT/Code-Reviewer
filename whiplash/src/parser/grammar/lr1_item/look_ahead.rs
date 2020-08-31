use std::fmt;
use crate::parser::node::nodetype::NodeType;
use crate::lexical_analyser::token::tokentype::TokenType;


pub enum Atom {
    Var(NodeType),
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
