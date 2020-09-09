use crate::lexical_analyser::token::token::Token;
use crate::parser::node::nodetype::NodeType;

pub struct Node {
    pub token: Token,
    pub category: NodeType,
    pub children: Vec< Box<Node> >,
}
