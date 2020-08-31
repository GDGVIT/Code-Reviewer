#[cfg(test)]
pub mod tests {
    use super::super::atom::{Atom, Atoms};
    use super::super::item::Item;
    use crate::parser::node::nodetype::NodeType;
    use crate::lexical_analyser::token::tokentype::TokenType;

    // To test debug outputs, do 
    // cargo test -- --nocapture
    #[test]
    fn check_atom_debug() {
        let a = Atoms::from(vec![Atom::Var(NodeType::and_expr), Atom::Dot, Atom::Tok(TokenType::BOOL)]);
        println!("{:?}", a);
    }

    #[test]
    fn check_item_debug() {
        let a = Atoms::from(vec![Atom::Var(NodeType::and_expr), Atom::Dot, Atom::Tok(TokenType::BOOL)]);
        let i = Item {
            lhs: NodeType::expr,
            rhs: a
        };
        println!("{:?}", i);
    }
}