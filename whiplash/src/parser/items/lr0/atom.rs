use std::fmt;
use std::hash::Hash;
use crate::grammar::production::Atom;

#[derive(Hash)]
pub struct Atoms {
    pub vals: Vec<Atom>,
    pub head: usize
}

impl Atoms {
    pub fn from(v: Vec<Atom>) -> Atoms {
        Atoms {
            vals: v,
            head: 0
        }
    }
}

impl fmt::Debug for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = "";
        for (i, atom) in self.vals.iter().enumerate() {
            if i == self.head {
                out = &format!("{} .", out);
            }
            out = &format!("{} {:?}", out, atom);
        }

        write!(f, "{}", out)
    }
}