use crate::lexical_analyser::token::token::Token;
use crate::grammar::Symbol;

pub struct Node {
    pub token: Token,
    pub category: Symbol,
    pub children: Vec< Box<Node> >,
}
