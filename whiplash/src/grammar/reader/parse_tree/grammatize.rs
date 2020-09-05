// Converts a generated parse tree into a grammar
use super::{Tree, Node, NodeType};
use crate::grammar::production::{Atom, Atoms};

impl Tree {
    pub fn to_rhs_list(current_node: Box<Node>) -> Vec<Atoms> {
        let out = vec![];
        for child in current_node.children.into_iter() {
            if let NodeType::X = child.nodetype {
                out = Self::permute_concat(out, Self::to_rhs_list(child));
            } else if format!("{:?}", child.atoms.vals) == "| ".to_string() {
                continue;
            } else {
                out = Self::permute_concat(out, vec![child.atoms]);
            }
        } 
        out
    }

    fn permute_concat(accumulated: Vec<Atoms>, tails: Vec<Atoms>) -> Vec<Atoms> {
        vec![]
    }
}