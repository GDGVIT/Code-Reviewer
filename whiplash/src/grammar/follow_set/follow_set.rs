use crate::grammar::Grammar;
use crate::grammar::production::Atom;
use std::collections::{HashMap, HashSet};
type AtomSet = HashSet<Atom>;

impl Grammar {
    pub fn get_FOLLOW(&self) -> HashMap<Atom, AtomSet> {
        let mut out = HashMap::new();

        for prod in self.iter() {
            for atom in prod.rhs {
                
            }
        }

        out
    }
}