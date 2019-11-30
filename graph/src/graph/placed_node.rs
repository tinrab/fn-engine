use std::collections::HashMap;

use crate::graph::property_value::PropertyValue;
use crate::schema::node::Node;
use crate::schema::property::{Property, PropertyId};

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

    pub fn get_property(&self, property_id: &PropertyId) -> &Property {
        self.node.properties.get(property_id).unwrap()
    }

    pub fn get_property_by_id(&self, property_id: &str) -> &Property {
        self.node
            .properties
            .get(&PropertyId::from(property_id))
            .unwrap()
    }
}

impl PartialEq for PlacedNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
