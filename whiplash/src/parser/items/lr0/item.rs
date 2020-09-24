use crate::grammar::Symbol;
use crate::grammar::production::Rule;
use super::atom::Atoms;
use std::fmt;

#[derive(Hash)]
pub struct Item {
    pub lhs: Symbol,
    pub rhs: Atoms
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.lhs, self.rhs)
    }
}

impl Item {
    pub fn production_to_initial_item(rule: &Rule) -> Self {
        let lhs = rule.start_symbol.clone();
        let rhs: Atoms = Atoms::from(rule.rhs.vals.clone());

        Item {
            lhs,
            rhs
        }
    }
}