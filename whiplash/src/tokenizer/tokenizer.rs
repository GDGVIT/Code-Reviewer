use crate::tokenizer::token::{Token, TokenType};

/// Tokenizer for Python
pub struct Tokenizer<'a> {
    keywords: Vec<&'a str>,
    operators: Vec<&'a str>,
    symbols_to_ignore: Vec<char>,
    symbols_to_include: Vec<&'a str>,
    parenthesis: Vec<&'a str>,
}

impl<'a> Tokenizer<'a> {
    
    /// Tokenizes a single line of code
    pub fn tokenize_line(&'a self, line: &'a str) -> Vec<Token> {
        if line.starts_with("#") {
            return vec![]
        }

        let mut result: Vec<Token> = Vec::new();
        for mut tokens in line
            .trim_end()
            .split(|c: char| c.is_whitespace() || self.symbols_to_ignore.contains(&c))
            .map(|x| self.identify_tokens(x)) {
                result.append(&mut tokens);
            }

        self.sanitize(result)
    }
    
    /// Classifies token according to TokenType
    /// tokenize_line() splits line by symbol/whitespace, but other tokens might be present
    /// identify_token() will further split the result into tokens and tokenize
    fn identify_tokens(&'a self, s: &'a str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        // Split a lexeme that might contain multiple lexemes
        let lexemes = self.split_lexeme(s);

        for lexeme in lexemes {
            // println!("{}", lexeme);
            // List of types: op, num, bool, keyword, id
            if lexeme.parse::<f64>().is_ok() {
                tokens.push(Token::new(TokenType::NUM, lexeme));

            } else if self.keywords.contains(&lexeme) {
                tokens.push(Token::new(TokenType::KEYWORD, lexeme));

            } else if ["True", "False"].contains(&lexeme) {
                tokens.push(Token::new(TokenType::BOOL, lexeme));

            } else if self.operators.contains(&lexeme) {
                tokens.push(Token::new(TokenType::OP, lexeme));

            } else if self.symbols_to_include.contains(&lexeme) {
                tokens.push(Token::new(TokenType::SYM, lexeme));

            } else if self.parenthesis.contains(&lexeme) {
                tokens.push(Token::new(TokenType::PAR, lexeme));

            } else if self.is_literal(&lexeme) {
                tokens.push(Token::new(TokenType::LIT, lexeme));

            } else {
                tokens.push(Token::new(TokenType::ID, lexeme));
            }
        }

        tokens
    }

    /// Merges two separate tokens if they should actually be a single token
    /// For instance, =, = is merged to ==
    fn sanitize(&'a self, tokens: Vec<Token<'a>>) -> Vec<Token> {
        let mut result = Vec::new();
        let mut this_is_pushed = false;
        
        for (i, token) in tokens.iter().enumerate() {
            if i == tokens.len() - 1 {
                result.push(*token);
                break;
            }
            if this_is_pushed {
                this_is_pushed = false;
                continue;
            }

            if let TokenType::OP = token.category {
                let next_tok = &tokens[i+1];

                if *token == *next_tok {
                    this_is_pushed = true;
                    if token.value == "/" {
                        result.push(Token::new(TokenType::OP, "//" ));
                        continue;

                    } else if token.value == "*" {
                        result.push(Token::new(TokenType::OP, "**"));
                        continue;

                    } else if token.value == "<" {
                        result.push(Token::new(TokenType::OP, "<<"));
                        continue;

                    } else if token.value == ">" {
                        result.push(Token::new(TokenType::OP, ">>"));
                        continue;

                    } else if token.value == "=" {
                        result.push(Token::new(TokenType::OP, "=="));
                        continue;

                    } else {
                        this_is_pushed = false;
                    }

                } else if next_tok.value == "=" {
                    this_is_pushed = true;
                    if token.value == "<" {
                        result.push(Token::new(TokenType::OP, "<="));
                        continue;

                    } else if token.value == ">" {
                        result.push(Token::new(TokenType::OP, ">="));
                        continue;

                    } else if token.value == "!" {
                        result.push(Token::new(TokenType::OP, "!="));
                        continue;

                    } else {
                        this_is_pushed = false;
                    }
                } 
            }

            result.push(*token);
        }

        result
    }

    fn is_literal(&'a self, s: &&str) -> bool {
        
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

    /// Splits a lexeme such as a+b into a, +, b
    /// Since it only inspects one element at a time, lexemes such as == will be split into two and need to be sanitized
    fn split_lexeme(&'a self, lexeme: &'a str) -> Vec<&'a str> {
        // Check symbols_to_include, operators, parentheses
        let token_checks = [
            &self.symbols_to_include[..],
            &self.operators[..],
            &self.parenthesis[..],
        ].concat();

        let one_char_tokens: Vec<&&str> = token_checks
            .iter()
            .filter(|x| x.chars().count() == 1)
            .collect();

        let mut result: Vec<&str> = Vec::new();
        let mut last = 0;
        let mut entered_loop = false;
        for (index, matched) in lexeme.match_indices(|c: char| one_char_tokens.contains(&&&c.to_string()[..])) {
            entered_loop = true;
            if last != index {
                result.push(&lexeme[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }

        if !entered_loop {
            result.push(lexeme);
        }

        result
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
        let symbols_to_ignore = vec![
            ',',
        ];

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
        ];

        Tokenizer {
            keywords, 
            operators, 
            symbols_to_ignore, 
            symbols_to_include, 
            parenthesis
        }
    }
    
}