use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::hash::Hash;

#[derive(PartialEq, Hash, Copy, Clone, Debug, EnumIter)]
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
