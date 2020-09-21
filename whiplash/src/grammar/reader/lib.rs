use crate::grammar::{Symbol, production::{self, Rule, Atom}, errors::InvalidError, reader::parse_tree::Tree, Grammar};
use crate::lexical_analyser::token::{Token, TokenType};
use std::fs;
use std::error::Error;
use regex::Regex;
use strum::IntoEnumIterator;

impl Grammar {
    /// Parse a grammar specification file into a grammar
    pub fn from_file(filename: String) -> Result<Grammar, Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        let lines: Vec<&str> = contents.lines().collect();
        let mut productions = vec![];

        for line in lines.into_iter() {
            productions.append(&mut Self::rules_from_line(line)?);
        }
        Ok(Grammar::from(productions))
    }

    /// Extract rules from a single line of input
    fn rules_from_line(line: &str) -> Result<Vec<Rule>, InvalidError> {
        // Regex to divide grammar notation into symbols. Based on 
        // https://stackoverflow.com/questions/6462578/regex-to-match-all-instances-not-inside-quotes
        let re = regex::Regex::new(r"'[^']+'|(:|\)\*|\]\*|\)|\]|\(|\[| |\|)").unwrap();
        let lexemes = Self::split_keep(&re, line);
        let atoms = production::Atoms::from( 
                    lexemes
                        .iter()
                        .map(|lexeme| Self::atomize(lexeme))
                        .collect()
        );

        if !Self::check_valid(&atoms) {
            return Err(InvalidError::from(line));
        }

        let rules_found = Self::atoms_to_rules(atoms);
        if let None = rules_found {
            return Err(InvalidError::from(line));
        }

        Ok(rules_found.unwrap())
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

    fn atomize(lexeme: &str) -> production::Atom {
        for ntype in Symbol::iter() {
            if stringify!(ntype) == lexeme {
                return production::Atom::Var(ntype);
            }
        }

        for ttype in TokenType::iter() {
            if stringify!(ttype) == lexeme {
                return production::Atom::TokType(ttype);
            }
        }

        if lexeme == format!("{:?}", production::Atom::Epsilon) {
            return production::Atom::Epsilon;
        }

        production::Atom::Tok(
            Token::from(lexeme.to_string())
        )
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

        let is_non_empty = atoms.vals.len() > 1;

        let closure_atoms = [Atom::from_token(")*".to_string()), Atom::from_token("]*".to_string())];
        let mut contains_closure = false;
        for atom in atoms.iter() {
            for closure_atom in &closure_atoms {
                if *closure_atom == atom {
                    contains_closure = true;
                }
            }
        }

        is_first_symbol && is_second_colon && is_non_empty && !contains_closure
    }

    fn atoms_to_rules(mut atoms: production::Atoms) -> Option<Vec<Rule>> {
        let first_symbol = atoms.vals.remove(0);

        let start_symbol = if let production::Atom::Var(sym) = first_symbol {
            sym
        } else {
            return None;
        };

        let rhs = production::Atoms::from(atoms.vals[1..].to_vec());
        let parse_tree = Tree::from(rhs);

        Some(parse_tree.get_rules(start_symbol))
    }
}
