use crate::grammar::Grammar;
use crate::grammar::production::Atom;
use std::collections::{HashMap, HashSet};
type AtomSet = HashSet<Atom>;

impl Grammar {
    pub fn get_FOLLOW(&self) -> HashMap<Atom, AtomSet> {
        let all_atoms = self.get_all_atoms();
        let mut out: HashMap<Atom, AtomSet> = HashMap::new();
        
        for atom in all_atoms {
            out.insert(atom, HashSet::new());
        }

        loop {
            let mut updated = false;
            for prod in self.iter() {
                for (i, atom) in prod.rhs.iter().enumerate() {

                    if let Atom::Term(_) = atom {
                        continue;
                    }

                    if i < prod.rhs.vals.len() - 1 {
                        let prev_len = out[&atom].len();
                        let next = &prod.rhs[i+1]; 
                        out.get_mut(&atom)
                            .unwrap()
                            .extend(self.FIRST[next].clone());
                        updated = prev_len != out[&atom].len();

                    } else {
                        let prev_len = out[&atom].len();
                        let key = Atom::from_symbol(&prod.start_symbol);
                        let val = out.get_mut(&key).unwrap().clone();
                        out.get_mut(&atom)
                            .unwrap()
                            .extend(val);
                        updated = prev_len != out[&atom].len();
                    }
                }
            }
            if !updated {
                break;
            }
        }
        out
    }
}