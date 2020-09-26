use crate::grammar::{production::Atom, Grammar};
use std::collections::{HashSet, HashMap};
type AtomSet = HashSet<Atom>;

impl Grammar {
    pub fn get_FIRST(&self) -> HashMap<Atom, AtomSet> {
        let all_atoms = self.get_all_atoms();
        let mut out = HashMap::new();

        for atom in all_atoms {
            if let Atom::Term(_) = atom {
                let mut singleton = HashSet::new();
                singleton.insert(atom.clone());
                out.insert(atom, singleton);
            } else {
                out.insert(atom, HashSet::new());
            }
        }

        loop {
            let mut updated = false;

            for rule in &self.productions {
                let index = &Atom::from_symbol(&rule.start_symbol);
                let prev_len = out[index].len();

                if let Atom::Term(_) = rule.rhs[0] {
                    out.get_mut(index)
                        .unwrap()
                        .insert(rule.rhs[0].clone());
                } else {
                    let first_rhs_index = &rule.rhs[0];
                    let rhs_set = out[first_rhs_index].clone(); 

                    out.get_mut(index)
                        .unwrap()
                        .extend(rhs_set);   
                }

                if prev_len < out[index].len() {
                    updated = true;
                }
            }

            if !updated {
                break;
            }
        }
        
        out
    }
}