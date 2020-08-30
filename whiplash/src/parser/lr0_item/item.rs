use crate::parser::node::nodetype::NodeType;
use super::atom::Atom;

pub struct Item {
    lhs: NodeType,
    rhs: Vec<Atom>
}

// impl fmt::Debug for Item {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, )
//     }
// }
