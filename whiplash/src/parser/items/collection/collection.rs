use super::super::lr1;
use std::hash::Hash;

#[derive(Hash)]
pub struct ItemCollection {
    pub items: Vec<lr1::Item>
}