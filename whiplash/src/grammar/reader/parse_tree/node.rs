use crate::grammar::production::{Atoms, Atom};
use super::helpers::{Delim, is_inv_paren};

pub enum NodeType {
    X,
    Leaf
}

pub struct Node {
    pub children: Vec<Box<Node>>,
    pub nodetype: NodeType,
    pub atoms: Atoms
}


impl Node {
    pub fn from(nt: NodeType, atoms: Atoms) -> Node {
        Node {
            children: vec![],
            nodetype: nt,
            atoms: atoms 
        }
    }

    pub fn new() -> Node {
        Node {
            children: vec![],
            nodetype: NodeType::X,
            atoms: Atoms::from(vec![])
        }
    }

    pub fn create_tree(node: Box<Node>) -> Box<Node> {
        let mut delim = Delim::new();
        let mut result = Node::new();
        let mut acc: Vec<Atom> = vec![];

        for atom in node.atoms.vals.into_iter() {
            let val = if let Some(val) = atom.get_token_value() {
                val
            } else {
                "".to_string()
            };

            if delim.count == 0 {
                if val == "(".to_string() || val == "[".to_string() {
                    delim.count += 1;
                    delim.paren = val.clone();

                    if val == "[".to_string() {
                        acc.push(Atom::from_token("(".to_string()));
                    }
                } else {
                    result.children.push(
                        Box::new(
                            Node::from(NodeType::Leaf, Atoms::from(vec![atom]))
                        )
                    )
                }
            } else {
                if val == delim.paren {
                    delim.count += 1;
                    acc.push(atom);

                } else if is_inv_paren(&delim.paren, &val) {
                    delim.count -= 1;
                    if delim.count == 0 {
                        if val == "]".to_string() {
                            acc.push(Atom::from_token(")".to_string()));
                            acc.push(Atom::from_token("|".to_string()));
                            acc.push(Atom::Epsilon);
                        }
                        result.children.push(
                            Box::new(
                                Node::from(NodeType::X, Atoms::from(acc))
                            )
                        );
                        acc = vec![];

                    } else {
                        acc.push(atom);

                    }
                } else {
                    acc.push(atom)
                }
            }
        }

        result.children = result.children
            .into_iter()
            .map(|child| {
                if let NodeType::X = child.nodetype {
                    return Self::create_tree(child);
                } else {
                    return child;
                }
            })
            .collect();

        Box::new(result)
    }
}

