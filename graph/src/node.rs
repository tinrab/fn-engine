use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use crate::property::{Property, PropertyId};

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

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub properties: HashMap<PropertyId, Property>,
}

impl<'a> Node {
    pub fn builder(id: NodeId) -> NodeBuilder {
        NodeBuilder::new(id)
    }
}

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

    pub fn property(&'a mut self, property: Property) -> &'a mut Self {
        if let Some(property) = self.node.properties.insert(property.id().clone(), property) {
            panic!("duplicate property id '{}'", property.id());
        }
        self
    }

    pub fn build(&'a self) -> Node {
        self.node.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Node, NodeId};
    use crate::property::{Property, PropertyId};

    #[test]
    fn basic() {
        let n1 = Node::builder(NodeId::from("a"))
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
