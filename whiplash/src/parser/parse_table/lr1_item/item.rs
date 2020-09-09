use crate::parser::parse_table::lr0_item;
use super::look_ahead;
use std::fmt;

pub struct Item {
    pub statement: lr0_item::Item,
    pub look_ahead:  look_ahead::Atoms
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.statement, self.look_ahead)
    }
}