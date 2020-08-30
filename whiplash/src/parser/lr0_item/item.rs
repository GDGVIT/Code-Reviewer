use crate::parser::node::nodetype::NodeType;
use super::atom::Atoms;
use std::fmt;

pub struct Item {
    pub lhs: NodeType,
    pub rhs: Atoms
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.lhs, self.rhs)
    }
}
