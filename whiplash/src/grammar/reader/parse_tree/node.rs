use crate::grammar::production::Atoms;
use super::helpers::Delim;

pub enum NodeType {
    X,
    Leaf
}

pub struct Node {
    children: Vec<Box<Node>>,
    nodetype: NodeType,
    val: Atoms
}


impl Node {
    pub fn create_tree(atoms: Atoms) -> Box<Node> {
        let delim = Delim::new();

        for atom in atoms.vals.iter() {
            if delim.count == 0 {
                
            }
        }
    }
}

