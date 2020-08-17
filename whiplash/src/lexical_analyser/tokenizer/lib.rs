use crate::lexical_analyser::tokenizer::tokenizer::*;
use crate::lexical_analyser::token::{token::*, tokentype::*};
use crate::lexical_analyser::tokenizer::helpers::*;

impl<'a> Tokenizer<'a> {
    /// Tokenizes a single line of code
    pub fn tokenize_line(&'a self, line: &'a str) -> Vec<Token> {
        if line.starts_with("#") {
            return vec![]
        }

        let mut result: Vec<Token> = Vec::new();
        
        for token in self
            .separate_lexemes(line)
            .iter()
            .map(|x| self.identify_token(x.to_string())) {
                result.push(token);
        } 
        
        // println!("{:?}", result);
        Self::sanitize(result)
    }

    fn separate_lexemes(&'a self, line: &'a str) -> Vec<String> {
        let mut buffer = String::from("");
        let mut result = Vec::new();
        let mut isl = InStringLit {
            in_string_literal: false,
            terminal: None,
        };

        let split_tokens = [
            &self.symbols_to_include[..],
            &self.operators[..],
            &self.parenthesis[..],
        ].concat();

        let separators_to_include: Vec<&&str> = split_tokens
            .iter()
            .filter(|x| x.chars().count() == 1)
            .collect();

        for c in line.chars() {
            if isl.in_string_literal {
                buffer.push(c);
                if c == isl.terminal.unwrap() {
                    isl.set(false, None);
                    result.push(buffer.clone());
                    buffer = "".to_string();
                }

            } else {
                if c == '\"' || c == '\'' {
                    isl.set(true, Some(c));
                    if buffer != "".to_string() {
                        result.push(buffer.clone());
                    }
                    buffer = c.to_string();

                } else if separators_to_include.contains(&&&c.to_string()[..]) {
                    if buffer != "".to_string() {
                        result.push(buffer.clone());
                    }
                    result.push(c.to_string());
                    buffer = "".to_string();

                } else if 
                    c.is_whitespace() ||
                    self.symbols_to_ignore.contains(&c) {
                        if buffer != "".to_string() {
                            result.push(buffer.clone());
                            buffer = "".to_string();
                        }

                } else {
                    buffer.push(c);

                }
            }
        }
        if buffer != "".to_string() {
            result.push(buffer.clone());
        }

        result
    }
    
    /// Classifies token according to TokenType
    fn identify_token(&'a self, lexeme: String) -> Token {
        let s = &lexeme[..];
        // println!("Lexeme: {}", lexeme);
        // List of types: op, num, bool, keyword, id
        if lexeme.parse::<f64>().is_ok() {
            return Token::new(TokenType::NUM, lexeme);

        } else if self.keywords.contains(&s) {
            return Token::new(TokenType::KEYWORD, lexeme);

        } else if ["True", "False"].contains(&s) {
            return Token::new(TokenType::BOOL, lexeme);

        } else if self.operators.contains(&s) {
            return Token::new(TokenType::OP, lexeme);

        } else if self.symbols_to_include.contains(&s) {
            return Token::new(TokenType::SYM, lexeme);

        } else if self.parenthesis.contains(&s) {
            return Token::new(TokenType::PAR, lexeme);

        } else if Self::is_literal(&s) {
            return Token::new(TokenType::LIT, lexeme);
        
        } else {
            return Token::new(TokenType::ID, lexeme);
        }
    }

    /// Merges two separate tokens if they should actually be a single token
    /// For instance, Token(OP, "="), Token(OP, "=") is merged to Token(OP, "==")
    fn sanitize(tokens: Vec<Token>) -> Vec<Token> {

        let mut result: Vec<Token> = Vec::new();
        let mut this_is_pushed = false;
        let total_tokens = tokens.len();
        
        for (i, token) in tokens.iter().enumerate() {
            // Token identification might erroneously include empty strings as tokens. 
            // Remove those
            if token.value == "" {
                continue;
            }

            // Last token does not merge with a succeeding token, simply push
            if i == total_tokens - 1 {
                result.push(token.clone());
                break;
            }

            // Skip token if it was pushed in previous iteration
            if this_is_pushed {
                this_is_pushed = false;
                continue;
            }

            let next_tok = &tokens[i+1];
            if let TokenType::OP = token.category {

                if *token == *next_tok {
                    this_is_pushed = true;
                    if token.value == "/" {
                        result.push(Token::new(TokenType::OP, "//" .to_string()));
                        continue;

                    } else if token.value == "*" {
                        result.push(Token::new(TokenType::OP, "**".to_string()));
                        continue;

                    } else if token.value == "<" {
                        result.push(Token::new(TokenType::OP, "<<".to_string()));
                        continue;

                    } else if token.value == ">" {
                        result.push(Token::new(TokenType::OP, ">>".to_string()));
                        continue;

                    } else if token.value == "=" {
                        result.push(Token::new(TokenType::OP, "==".to_string()));
                        continue;

                    } else {
                        this_is_pushed = false;
                    }

                } else if next_tok.value == "=" {
                    this_is_pushed = true;
                    if token.value == "<" {
                        result.push(Token::new(TokenType::OP, "<=".to_string()));
                        continue;

                    } else if token.value == ">" {
                        result.push(Token::new(TokenType::OP, ">=".to_string()));
                        continue;

                    } else {
                        this_is_pushed = false;
                    }
                } 
            } else if token.value == "!" && next_tok.value == "=" {
                this_is_pushed = true;
                result.push(Token::new(TokenType::OP, "!=".to_string()));
                continue;
            }

            result.push(token.clone());
        }

        result
    }
}
