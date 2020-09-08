use super::Atom;
use std::fmt;

pub struct Atoms {
    pub vals: Vec<Atom>
}

impl Atoms {
    pub fn from(v: Vec<Atom>) -> Atoms {
        Atoms {
            vals: v
        }
    }

    pub fn from_single_token(tok: String) -> Atoms {
        Atoms {
            vals: vec![Atom::from_token(tok)]
        }
    }

    pub fn from_single_atom(a: Atom) -> Atoms {
        Atoms {
            vals: vec![a]
        }
    }
}

impl fmt::Debug for Atoms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self.vals.iter().fold(
                String::new(), 
                |acc, atom| acc + &format!("{:?}", &atom)[..] + " "
            )
        )
    }
}

impl PartialEq for Atoms {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}