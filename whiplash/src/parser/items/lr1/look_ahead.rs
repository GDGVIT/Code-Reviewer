use std::fmt;
use std::hash::Hash;
use crate::grammar::production::Atom;

#[derive(Hash)]
pub struct Atoms {
    vals: Vec<Atom>
}

impl fmt::Debug for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self.vals.iter().fold(
                String::new(),
                |acc, atom| acc + "/" + &format!("{:?}", atom)[..]
            )
        )
    }
}

impl Atoms {
    pub fn from(vals: Vec<Atom>) -> Atoms {
        Atoms {
            vals
        }
    }
}