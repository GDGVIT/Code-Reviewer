use super::super::lr0;
use super::look_ahead;
use crate::grammar::production::Rule;
use std::fmt;

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

impl Item {
    pub fn rule_to_initial_item(rule: &Rule) -> Item {
        let statement = lr0::Item::production_to_initial_item(rule);

        Item {
            statement,
            look_ahead: look_ahead::Atoms::from(vec![])
        }
    }
}