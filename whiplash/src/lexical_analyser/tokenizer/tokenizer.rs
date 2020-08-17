// use crate::lexical_analyser::token::{token::*, tokentype::*};


/// Tokenizer for Python
pub struct Tokenizer<'a> {
    pub keywords: Vec<&'a str>,
    pub operators: Vec<&'a str>,
    pub symbols_to_ignore: Vec<char>,
    pub symbols_to_include: Vec<&'a str>,
    pub parenthesis: Vec<&'a str>,
    pub whitespaces: Vec<&'a str>,
}

impl<'a> Tokenizer<'a> {

    pub fn is_literal(s: &&str) -> bool {
        
        if (s.starts_with("\"") || 
            s.starts_with("b\"") || 
            s.starts_with("r\"") || 
            s.starts_with("u\""))
            && s.ends_with("\"") {
                return true;

        } else if (s.starts_with("\'") || 
            s.starts_with("b\'") || 
            s.starts_with("r\'") || 
            s.starts_with("u\'"))
            && s.ends_with("\'") {
                return true;

        }

        false
    }

    /// Instantiates a Tokenizer
    pub fn new() -> Tokenizer<'a> {
        // List of keywords used in Python. Can be found by doing keyword.kwlist in py
        let keywords = vec![ 
            "None", 
            "and", 
            "as", 
            "assert", 
            "async", 
            "await", 
            "break", 
            "class", 
            "continue", 
            "def", 
            "del", 
            "elif", 
            "else", 
            "except", 
            "finally", 
            "for", 
            "from", 
            "global", 
            "if", 
            "import", 
            "in", 
            "is", 
            "lambda", 
            "nonlocal", 
            "not", 
            "or", 
            "pass", 
            "raise", 
            "return", 
            "try", 
            "while", 
            "with", 
            "yield"
        ];

        // List of operators in Python
        let operators = vec![
            "+",
            "-",
            "*",
            "/",
            "//",
            "&",
            "^",
            "~",
            "|",
            "**",
            "<<",
            ">>",
            "%",
            "<",
            ">",
            "=",
            "<=",
            ">=",
            "==",
            "!=",
        ];

        // List of symbols
        let symbols_to_ignore = vec![];

        let parenthesis = vec![
            "(",
            ")",
            "{",
            "}",
            "[",
            "]",
        ];

        let symbols_to_include = vec![
            ":",
            "\'\'\'",
            ",",
        ];

        let whitespaces = vec![
            " ",
            "\t",
            "\n",
            "\r",
            // "\v", // \x0b
            // "\f", // \x0c
        ];

        Tokenizer {
            keywords, 
            operators, 
            symbols_to_ignore, 
            symbols_to_include, 
            parenthesis,
            whitespaces,
        }
    }
    
}
