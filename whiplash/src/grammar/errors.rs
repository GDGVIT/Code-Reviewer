use std::{error::Error, fmt};

#[derive(Debug)]
pub struct InvalidError {
    invalid_production: String
}

impl Error for InvalidError {}

impl fmt::Display for InvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid production - {}", self.invalid_production)
    }
}

impl InvalidError {
    pub fn from(s: &str) -> Self {
        InvalidError {
            invalid_production: String::from(s)
        }
    }
}