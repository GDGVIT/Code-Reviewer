use crate::lexical_analyser::token::token::Token;

pub struct Node {
    pub token: Token,
    pub right: Box<Node>,
    pub left: Box<Node>,
}
