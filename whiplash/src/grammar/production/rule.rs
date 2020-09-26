use crate::grammar::Symbol;
use super::Atoms;
use std::fmt;

#[derive(Clone)]
pub struct Rule {
    pub start_symbol: Symbol,
    pub rhs: Atoms
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.start_symbol, self.rhs)
    }
}

impl Rule {
    pub fn from(start_symbol: Symbol, rhs: Atoms) -> Rule {
        Rule {
            start_symbol,
            rhs
        }
    }
}