//! Placed node is an instance of a schema node.

use std::collections::HashMap;

use crate::graph::property_value::PropertyValue;
use crate::schema::node::Node;
use crate::schema::property::{Property, PropertyId};

/// Represents a placed node inside an graph.
#[derive(Debug, Clone)]
pub struct PlacedNode {
    /// Schema of a node.
    pub node: Node,
    /// Unique key.
    pub key: String,
    /// Assigned or default values for this node instance.
    pub values: HashMap<PropertyId, PropertyValue>,
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
                        property.id(),
                        match property {
                            Property::Input { default_value, .. } => default_value.clone(),
                            _ => None,
                        },
                    )
                })
                .filter(|(_, default_value)| default_value.is_some())
                .map(|(property_id, default_value)| {
                    (
                        property_id.clone(),
                        PropertyValue::new(property_id.clone(), default_value.unwrap()),
                    )
                })
                .collect(),
        }
    }

    /// Returns node's property.
    /// # Panics
    /// If propert does not exist.
    pub fn get_property(&self, property_id: &PropertyId) -> &Property {
        self.node.properties.get(property_id).unwrap()
    }

    /// Returns node's property by string id.
    /// # Panics
    /// If propert does not exist.
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
