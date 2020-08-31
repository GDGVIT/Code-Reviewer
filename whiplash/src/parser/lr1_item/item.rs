use crate::parser::lr0_item;
use super::look_ahead;

pub struct Item {
    pub statement: lr0_item::Item,
    pub look_ahead:  look_ahead::Atoms
}