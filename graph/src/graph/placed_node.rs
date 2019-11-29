use std::collections::HashMap;

use crate::graph::property_value::PropertyValue;
use crate::schema::node::Node;
use crate::schema::property::PropertyId;

#[derive(Debug, Clone)]
pub struct PlacedNode {
    pub node: Node,
    pub key: String,
    pub values: HashMap<PropertyId, PropertyValue>,
}

impl PlacedNode {
    pub fn new(node: &Node, key: &str) -> Self {
        PlacedNode {
            node: node.clone(),
            key: key.into(),
            values: Default::default(),
        }
    }
}

impl PartialEq for PlacedNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
