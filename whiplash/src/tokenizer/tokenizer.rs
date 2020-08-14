use crate::tokenizer::token::{Token, TokenType};

/// Tokenizer for Python
pub struct Tokenizer<'a> {
    keywords: Vec<&'a str>,
    operators: Vec<&'a str>,
    symbols: Vec<char>,
}

impl<'a> Tokenizer<'a> {
    
    /// Tokenizes a single line of code
    pub fn tokenize_line(&'a self, line: &'a str) -> Vec<Token> {
        if line.starts_with("#") {
            return vec![]
        }

        line
            .split(|c: char| c.is_whitespace() || self.symbols.contains(&c))
            .map(|x| self.identify_tokens(x))
            .collect()
    }
    
    /// Classifies token according to TokenType
    /// tokenize_line() splits line by symbol/whitespace, but other tokens might be present
    /// identify_token() will further split the result into tokens and tokenize
    fn identify_tokens(&'a self, s: &'a str) -> Token {
        let tokens: Vec<Token> = Vec::new();
        


        // List of types: var, op, num, bool, keyword, id
        if s.parse::<f64>().is_ok() {
            return Token::new(TokenType::NUM, s);
        } else if self.keywords.contains(&s) {
            return Token::new(TokenType::KEYWORD, s);
        } else if ["True", "False"].contains(&s) {
            return Token::new(TokenType::BOOL, s);
        } else if self.operators.contains(&s) {
            return Token::new(TokenType::OP, s);
        }
        Token::new(TokenType::ID, s)
    }

    fn split_lexeme(lexeme: &'a str) -> Vec<&'a str> {
        vec![]
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
        let symbols = vec![
            ',',
            '(',
            ')'
        ];

        Tokenizer {keywords, operators, symbols}
    }
    
}