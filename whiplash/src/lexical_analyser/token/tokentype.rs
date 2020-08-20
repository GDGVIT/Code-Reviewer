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
