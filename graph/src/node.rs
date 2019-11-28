use std::collections::HashMap;

use crate::property::{Property, PropertyId};

#[derive(Debug, Hash, Clone, PartialOrd, Eq, PartialEq)]
pub struct NodeId(String);

impl From<&str> for NodeId {
    fn from(s: &str) -> Self {
        NodeId(String::from(s))
    }
}

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    properties: HashMap<PropertyId, Property>,
}

#[cfg(test)]
mod tests {
    use crate::node::{Node, NodeId};
    use crate::property::{Property, PropertyId};

    #[test]
    fn basic() {
        let n1 = Node {
            id: "a".into(),
            properties: [
                (PropertyId::from("command"), Property::Command { id: "command".into() }),
            ].iter().cloned().collect(),
        };

        assert_eq!(n1.id, NodeId::from("a"));
        assert_eq!(Property::Command { id: "command".into() }, *n1.properties.get(&PropertyId::from("command")).unwrap())
    }
}
