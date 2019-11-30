//! Types for defining nodes.

use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use crate::schema::property::{Property, PropertyId};
use crate::value::{DataType, Value};

/// Type for node ids.
#[derive(Debug, Hash, Clone, PartialOrd, Eq, PartialEq)]
pub struct NodeId(String);

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.0.as_str())
    }
}

impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        NodeId(String::from(s))
    }
}

/// Describes a node.
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique id of a node.
    pub id: NodeId,
    /// Node's properties by ids.
    pub properties: HashMap<PropertyId, Property>,
}

impl<'a> Node {
    /// Constructs a `NodeBuilder`.
    pub fn builder(id: &'a str) -> NodeBuilder {
        NodeBuilder::new(NodeId::from(id))
    }
}

/// Utility for building nodes.
pub struct NodeBuilder {
    node: Node,
}

impl<'a> NodeBuilder {
    fn new(id: NodeId) -> Self {
        NodeBuilder {
            node: Node {
                id,
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
        self.property(Property::Command {
            id: PropertyId::from(id),
        })
    }

    /// Declares a new event property.
    pub fn event(&'a mut self, id: &str) -> &'a mut Self {
        self.property(Property::Event {
            id: PropertyId::from(id),
        })
    }

    /// Declares a new input property.
    pub fn input(
        &'a mut self,
        id: &str,
        data_type: DataType,
        default_value: Option<Value>,
    ) -> &'a mut Self {
        self.property(Property::Input {
            id: PropertyId::from(id),
            data_type,
            default_value,
        })
    }

    /// Declares a new output property.
    pub fn output(&'a mut self, id: &str, data_type: DataType) -> &'a mut Self {
        self.property(Property::Output {
            id: PropertyId::from(id),
            data_type,
        })
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
        let n1 = Node::builder("a")
            .property(Property::Command {
                id: "command".into(),
            })
            .build();

        assert_eq!(n1.id, NodeId::from("a"));
        assert_eq!(
            Property::Command {
                id: "command".into()
            },
            *n1.properties.get(&PropertyId::from("command")).unwrap()
        )
    }
}
