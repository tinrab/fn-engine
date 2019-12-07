//! Placed node is an instance of a schema node.

use std::collections::HashMap;

use crate::graph::property_value::PropertyValue;
use crate::schema::node::Node;
use crate::schema::property::Property;
use crate::value::Value;

/// Represents a placed node inside an graph.
#[derive(Debug, Clone)]
pub struct PlacedNode {
    /// Schema of a node.
    pub node: Node,
    /// Unique key.
    pub key: String,
    /// Assigned or default values for this node instance.
    pub values: HashMap<String, PropertyValue>,
}

impl PlacedNode {
    /// Constructs a new `PlacedNode`.
    pub fn new(node: &Node, key: &str) -> Self {
        PlacedNode {
            node: node.clone(),
            key: key.into(),
            values: node
                .properties
                .values()
                .filter(|property| property.is_input())
                .map(|property| {
                    (
                        property.id().clone(),
                        PropertyValue::new(
                            property.id(),
                            Value::default_for(*property.data_type().unwrap()),
                        ),
                    )
                })
                .collect(),
        }
    }

    /// Returns node's property.
    /// # Panics
    /// If propert does not exist.
    pub fn get_property(&self, property_id: &str) -> &Property {
        self.node
            .properties
            .get(&String::from(property_id))
            .unwrap()
    }
}

impl PartialEq for PlacedNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
