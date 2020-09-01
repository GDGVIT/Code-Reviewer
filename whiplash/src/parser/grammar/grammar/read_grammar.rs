use super::Grammar;
use crate::parser::grammar::rule::Rule;
use std::fs;
use std::error::Error;
use regex::Regex;

impl Grammar {
    pub fn from_file(filename: String) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        let lines: Vec<&str> = contents.lines().collect();

        Ok(())
    }

    fn rules_from_line(line: &str) -> Vec<Rule> {
        let rules_found = vec![];
        let re = regex::Regex::new(r":|\)\*|\]\*|\)|\]|\(|\[| |\|").unwrap();

        rules_found
    }

    fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
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
    
}
