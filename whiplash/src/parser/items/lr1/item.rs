use super::super::lr0;
use super::look_ahead;
use std::fmt;
use std::hash::Hash;

#[derive(Hash)]
pub struct Item {
    pub statement: lr0::Item,
    pub look_ahead:  look_ahead::Atoms
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.statement, self.look_ahead)
    }
}