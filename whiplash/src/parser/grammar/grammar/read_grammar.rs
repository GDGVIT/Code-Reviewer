use super::Grammar;
use crate::parser::grammar::rule::Rule;
use std::fs;
use std::error::Error;
use regex::Regex;

impl Grammar {
    /// Parse a grammar specification file into a grammar
    pub fn from_file(filename: String) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(filename)?;

        let lines: Vec<&str> = contents.lines().collect();

        Ok(())
    }

    /// Extract rules from a single line of input
    fn rules_from_line(line: &str) -> Vec<Rule> {
        let rules_found = vec![];

        // Regex to divide grammar notation into symbols. Based on 
        // https://stackoverflow.com/questions/6462578/regex-to-match-all-instances-not-inside-quotes
        let re = regex::Regex::new(r"'[^']+'|(:|\)\*|\]\*|\)|\]|\(|\[| |\|)").unwrap();
        let tokenized_line = Self::split_keep(&re, line);
        rules_found
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

    
}
