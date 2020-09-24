use crate::grammar::Symbol;
use super::atom::{Atoms, Atom};
use std::fmt;
use crate::grammar::production::Rule;

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
        let mut rhs = vec![Atom::Dot];

        for val in rule.rhs.vals.iter() {
            rhs.push(Atom::Val(val.clone()));
        }

        let rhs = Atoms::from(rhs);

        Item {
            lhs,
            rhs
        }
    }
}