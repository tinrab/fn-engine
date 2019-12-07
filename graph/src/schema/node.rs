//! Types for defining nodes.

use std::collections::HashMap;

use crate::schema::property::{
    CommandProperty, EventProperty, InputProperty, OutputProperty, Property,
};
use crate::value::DataType;

/// Describes a node.
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique id of a node.
    pub id: String,
    /// Node's properties by ids.
    pub properties: HashMap<String, Property>,
}

impl<'a> Node {
    /// Constructs a `NodeBuilder`.
    pub fn builder(id: &str) -> NodeBuilder {
        NodeBuilder::new(id)
    }
}

/// Utility for building nodes.
pub struct NodeBuilder {
    node: Node,
}

impl<'a> NodeBuilder {
    fn new(id: &str) -> Self {
        NodeBuilder {
            node: Node {
                id: String::from(id),
                properties: Default::default(),
            },
        }
    }

    /// Declares a new property.
    pub fn property(&'a mut self, property: Property) -> &'a mut Self {
        if let Some(property) = self.node.properties.insert(property.id().clone(), property) {
            panic!("duplicate property id '{}'", property.id());
        }
        self
    }

    /// Declares a new command property.
    pub fn command(&'a mut self, id: &str) -> &'a mut Self {
        self.property(Property::Command(CommandProperty {
            id: String::from(id),
        }))
    }

    /// Declares a new event property.
    pub fn event(&'a mut self, id: &str) -> &'a mut Self {
        self.property(Property::Event(EventProperty {
            id: String::from(id),
        }))
    }

    /// Declares a new input property.
    pub fn input(&'a mut self, id: &str, data_type: DataType) -> &'a mut Self {
        self.property(Property::Input(InputProperty {
            id: String::from(id),
            data_type,
        }))
    }

    /// Declares a new output property.
    pub fn output(&'a mut self, id: &str, data_type: DataType) -> &'a mut Self {
        self.property(Property::Output(OutputProperty {
            id: String::from(id),
            data_type,
        }))
    }

    /// Builds a `Node`.
    pub fn build(&'a self) -> Node {
        self.node.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let n1 = Node::builder("a").command("command").build();

        assert_eq!(n1.id, String::from("a"));
        assert_eq!(
            Property::Command(CommandProperty {
                id: String::from("command"),
            }),
            *n1.properties.get(&String::from("command")).unwrap()
        )
    }
}
