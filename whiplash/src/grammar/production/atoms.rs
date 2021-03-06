use super::Atom;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Atoms {
    pub vals: Vec<Atom>
}

// Iterator for Atoms
pub struct AtomsIter<T> {
    it: T
}

impl Atoms {
    pub fn from(v: Vec<Atom>) -> Atoms {
        Atoms {
            vals: v
        }
    }

    pub fn from_single_token(tok: String) -> Atoms {
        Atoms {
            vals: vec![Atom::from_token(&tok)]
        }
    }

    pub fn from_single_atom(a: Atom) -> Atoms {
        Atoms {
            vals: vec![a]
        }
    }

    pub fn iter<'a>(&'a self) -> AtomsIter<impl Iterator<Item=Atom> + 'a> {
        AtomsIter {
            it: self.vals.iter().cloned()
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

impl IntoIterator for Atoms {
    type Item = Atom;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vals.into_iter()
    }
}
 
impl<T> Iterator for AtomsIter<T>
where
    T: Iterator<Item = Atom>
{
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

impl Index<usize> for Atoms {
    type Output = Atom;

    fn index(&self, i: usize) -> &Self::Output {
        &self.vals[i]
    }
}

impl IndexMut<usize> for Atoms {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.vals[i]
    }
}