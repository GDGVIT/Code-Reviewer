use super::super::lr1;
use std::collections::BTreeSet;

#[derive(Hash)]
pub struct ItemCollection {
    pub items: BTreeSet<lr1::Item>
}