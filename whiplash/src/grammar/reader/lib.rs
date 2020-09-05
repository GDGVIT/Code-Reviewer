use crate::grammar::{production::{self, Rule}, errors::InvalidError, Grammar};
use crate::parser::node::NodeType;
use crate::lexical_analyser::token::{Token, TokenType};
use std::fs;
use std::error::Error;
use regex::Regex;
use strum::IntoEnumIterator;

impl Grammar {
    /// Parse a grammar specification file into a grammar
    pub fn from_file(filename: String) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        let lines: Vec<&str> = contents.lines().collect();

        Ok(())
    }

    /// Extract rules from a single line of input
    fn rules_from_line(line: &str) -> Result<Vec<Rule>, InvalidError> {
        let rules_found = vec![];

        // Regex to divide grammar notation into symbols. Based on 
        // https://stackoverflow.com/questions/6462578/regex-to-match-all-instances-not-inside-quotes
        let re = regex::Regex::new(r"'[^']+'|(:|\)\*|\]\*|\)|\]|\(|\[| |\|)").unwrap();
        let lexemes = Self::split_keep(&re, line);
        let lexemes: Vec<Option<_>> = lexemes
                        .iter()
                        .map(|lexeme| Self::atomize(lexeme))
                        .collect();

        if lexemes.iter().any(|&x| if let None = x {true} else {false}) {
            return Err(InvalidError::from(line));
        }

        let atoms = production::Atoms::from(
            lexemes.iter().map(|x| x.unwrap()).collect()
        );

        if !Self::check_valid(&atoms) {
            return Err(InvalidError::from(line));
        }

        
        Ok(rules_found)
    }

    /// Splits string according to regex and includes delimiters in output
    /// Ignores whitespaces
    fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
        // Based on https://stackoverflow.com/questions/56921637/how-do-i-split-a-string-using-a-rust-regex-and-keep-the-delimiters
        let mut result = Vec::new();
        let mut last = 0;
        for mat in r.find_iter(text) {
            let index = mat.start();
            let matched = mat.as_str();
            if !matched.trim().is_empty() {
                if last != index {
                    if !&text[last..index].trim().is_empty() {
                        result.push(&text[last..index]);
                    }
                }
                result.push(matched);
                last = index + matched.len();
            }
        }
        if last < text.len() {
            result.push(&text[last..]);
        }
        result
    }

    fn atomize(lexeme: &str) -> Option<production::Atom> {
        for ntype in NodeType::iter() {
            if stringify!(ntype) == lexeme {
                return Some(production::Atom::Var(ntype));
            }
        }

        for ttype in TokenType::iter() {
            if stringify!(ttype) == lexeme {
                return Some(production::Atom::TokType(ttype));
            }
        }

        if lexeme.starts_with('\'') && lexeme.ends_with('\'') {
            // Take string inside quotes
            let s: String = lexeme
                                .chars()
                                .skip(1)
                                .take(lexeme.len() - 2)
                                .collect();

            return Some(
                production::Atom::Tok(
                    Token::from(s)
                )
            )
        }
        
        None
    } 

    fn check_valid(atoms: &production::Atoms) -> bool {
        let is_first_symbol = match atoms.vals[0] {
            production::Atom::Var(_) => true,
            _ => false
        };

        let is_second_colon = match &atoms.vals[1] {
            production::Atom::Tok(t) => {
                if *t == Token::from(":".to_string()) {
                    true
                } else {
                    false
                }
            }
            _ => false
        };

        let is_non_empty = atoms.vals.len() > 2;

        is_first_symbol && is_second_colon && is_non_empty
    }

    fn atoms_to_rules(atoms: production::Atoms) -> Option<Vec<Rule>> {
        let start_symbol = if let production::Atom::Var(sym) = atoms.vals[0] {
            sym
        } else {
            return None;
        };

        let rhs = production::Atoms::from(
            atoms.vals[2..].to_vec()
        );

        vec![
            Rule::from(
                start_symbol, 
                rhs
            )
        ]
    }
}