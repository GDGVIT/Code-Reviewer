#[derive(Debug)]
#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    OP,         // Operator
    NUM,        // Numeric value
    BOOL,       // Boolean
    KEYWORD,    // Keyword
    ID,         // Identifier
    LIT,        // Any kind of literal
    SYM,        // Symbol (such as ',' and ':')
    PAR,        // Parenthesis
}

#[derive(Debug)]
#[derive(Copy, Clone)]
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

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.value == other.value
    }
}


// // ** << >> <= >= !=