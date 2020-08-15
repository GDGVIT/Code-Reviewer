use crate::lexical_analyser::token::*;

pub struct Node {
    // pub token: Box<Token>,
    pub right: Box<Node>,
    pub left: Box<Node>,
}
