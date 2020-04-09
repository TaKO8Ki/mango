use crate::domain::entity::dom::Node;
use crate::domain::entity::style::Value;
use std::collections::HashMap;

pub type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

#[derive(PartialEq)]
pub enum Display {
    Inline,
    Block,
    None,
}
