use super::{Node, NodeType};
use crate::grammar::production::Atoms;

pub struct Tree {
    pub root: Box<Node>
}

impl Tree {
    pub fn from(atoms: Atoms) -> Tree {
        let root = Node::from(
            NodeType::X,
            atoms
        );

        let root = Box::new(root);

        Tree {
            root: Node::create_tree(root)
        }
    }
}