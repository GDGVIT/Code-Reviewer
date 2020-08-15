use crate::lexical_analyser::tokenizer::tokenizer::*;
use crate::lexical_analyser::token::{token::*, tokentype::*};

impl<'a> Tokenizer<'a> {
    /// Tokenizes a single line of code
    pub fn tokenize_line(&'a self, line: &'a str) -> Vec<Token> {
        if line.starts_with("#") {
            return vec![]
        }

        let mut result: Vec<Token> = Vec::new();
        for mut tokens in line
            .trim()
            .split(|c: char| c.is_whitespace() || self.symbols_to_ignore.contains(&c))
            .map(|x| self.identify_tokens(x)) {
                result.append(&mut tokens);
        }
        
        // println!("{:?}", result);
        Self::sanitize(result)
    }
    
    /// Classifies token according to TokenType
    fn identify_tokens(&'a self, s: &'a str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        // Split a lexeme that might contain multiple lexemes
        let lexemes = self.split_lexeme(s);
        // println!("Lexemes are: {:?}", lexemes);

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

            } else if Self::is_literal(&lexeme) {
                tokens.push(Token::new(TokenType::LIT, lexeme));
            
            } else {
                tokens.push(Token::new(TokenType::ID, lexeme));
            }
            // println!("Token pushed into tokens list: {:?}", tokens.last());
        }
        tokens
    }

    /// Merges two separate tokens if they should actually be a single token
    /// For instance, Token(OP, "="), Token(OP, "=") is merged to Token(OP, "==")
    fn sanitize(tokens: Vec<Token<'a>>) -> Vec<Token> {
        let mut result = Vec::new();
        let mut this_is_pushed = false;
        
        for (i, token) in tokens.iter().enumerate() {
            // Token identification might erroneously include empty strings as tokens. 
            // Remove those
            if token.value == "" {
                continue;
            }

            // Last token does not merge with a succeeding token, simply push
            if i == tokens.len() - 1 {
                result.push(*token);
                break;
            }

            // Skip token if it was pushed in previous iteration
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

    /// Splits a lexeme such as a+b into a, +, b
    /// 
    /// tokenize_line() splits line by symbol/whitespace, which mean lexemes that are not space separated need to be separated.
    /// This function inspects only one element at a time, lexemes such as == will be split into two and need to be sanitized
    /// Using Tokenizer::sanitize()
    pub fn split_lexeme(&'a self, lexeme: &'a str) -> Vec<&'a str> {
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
        // println!("Lexeme: {:?}", lexeme);
        for (index, matched) in lexeme.match_indices(|c: char| one_char_tokens.contains(&&&c.to_string()[..])) {
            if last != index {
                result.push(&lexeme[last..index]);
                // println!("Part before match: {}", &lexeme[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
            // println!("Matched part: {}", matched);
        }

        result.push(&lexeme[last..]);
        result
    }
}