#[derive(Debug)]
pub enum TokenType {
    OP,         // Operator
    NUM,        // Numeric value
    BOOL,       // Boolean
    KEYWORD,    // Keyword
    ID,         // Identifier
}

#[derive(Debug)]
pub struct Token<'a> {
    pub category: TokenType,
    pub value: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(category: TokenType, value: &'a str) -> Token {
        Token {
            category,
            value
        }
    }
}
