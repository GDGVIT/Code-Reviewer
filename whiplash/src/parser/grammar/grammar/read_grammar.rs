use super::Grammar;
use crate::parser::grammar::rule;
use crate::parser::node::nodetype::NodeType;
use crate::lexical_analyser::token::token::Token;
use std::fs;
use std::error::Error;
use regex::Regex;
use strum::IntoEnumIterator;
use super::errors::InvalidError;

impl Grammar {
    /// Parse a grammar specification file into a grammar
    pub fn from_file(filename: String) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        let lines: Vec<&str> = contents.lines().collect();

        Ok(())
    }

    /// Extract rules from a single line of input
    fn rules_from_line(line: &str) -> Result<Vec<rule::Rule>, InvalidError> {
        let rules_found = vec![];

        // Regex to divide grammar notation into symbols. Based on 
        // https://stackoverflow.com/questions/6462578/regex-to-match-all-instances-not-inside-quotes
        let re = regex::Regex::new(r"'[^']+'|(:|\)\*|\]\*|\)|\]|\(|\[| |\|)").unwrap();
        let lexemes = Self::split_keep(&re, line);
        let atoms = rule::Atoms::from(
            lexemes
                .iter()
                .map(|lexeme| Self::atomize(lexeme))
                .collect()
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

    fn atomize(lexeme: &str) -> rule::Atom {
        for ntype in NodeType::iter() {
            if stringify!(ntype) == lexeme {
                return rule::Atom::Var(ntype);
            }
        }

        rule::Atom::Tok(
            Token::from(String::from(lexeme))
        )
    } 

    fn check_valid(atoms: &rule::Atoms) -> bool {
        let is_first_symbol = match atoms.vals[0] {
            rule::Atom::Var(_) => true,
            rule::Atom::Tok(_) => false
        };

        let is_second_colon = match &atoms.vals[1] {
            rule::Atom::Tok(t) => {
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

    fn atoms_to_rules(atoms: rule::Atoms) -> Option<Vec<rule::Rule>> {
        let start_symbol = if let rule::Atom::Var(sym) = atoms.vals[0] {
            sym
        } else {
            return None;
        };

        let rhs = rule::Atoms::from(
            atoms.vals[2..].to_vec()
        );

        vec![
            rule::Rule::from(
                start_symbol, 
                rhs
            )
        ]
    }
}
