use crate::parser::node::nodetype::NodeType;
use super::Atoms;
use std::fmt;

pub struct Rule {
    pub start_symbol: NodeType,
    pub rhs: Atoms
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.start_symbol, self.rhs)
    }
}

impl Rule {
    pub fn from(start_symbol: NodeType, rhs: Atoms) -> Rule {
        Rule {
            start_symbol,
            rhs
        }
    }
}