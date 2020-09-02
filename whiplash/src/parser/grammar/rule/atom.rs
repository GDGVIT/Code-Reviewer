use std::fmt;
use crate::parser::node::nodetype::NodeType;
use crate::lexical_analyser::token::token::Token;

#[derive(Clone)]
pub enum Atom {
    Var(NodeType),
    Tok(Token)
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
    pub vals: Vec<Atom>
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