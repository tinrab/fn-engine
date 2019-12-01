//! Schema describes what elements can graphs contain.

use std::collections::HashMap;

use crate::schema::node::{Node, NodeId};

pub mod node;
pub mod property;

/// Holds available elements for building graphs.
#[derive(Debug, Clone)]
pub struct Schema {
    /// All available nodes.
    pub nodes: HashMap<NodeId, Node>,
}

/// Utility for building a `Schema`.
pub struct SchemaBuilder {
    schema: Schema,
}

impl Schema {
    /// Constructs a new `SchemaBuilder`.
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::new()
    }
}

impl<'a> SchemaBuilder {
    fn new() -> Self {
        SchemaBuilder {
            schema: Schema {
                nodes: Default::default(),
            },
        }
    }

    /// Declares a node.
    pub fn node(&'a mut self, node: Node) -> &'a mut SchemaBuilder {
        if let Some(node) = self.schema.nodes.insert(node.id.clone(), node) {
            panic!("duplicate node id '{}'", node.id);
        }
        self
    }

    /// Builds schema.
    pub fn build(&'a self) -> Schema {
        self.schema.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::property::{Property, PropertyId};
    use crate::value::DataType;

    use super::*;

    #[test]
    fn basic() {
        let schema = Schema::builder()
            .node(
                Node::builder("action")
                    .property(Property::Event {
                        id: PropertyId::from("executed"),
                    })
                    .build(),
            )
            .node(
                Node::builder("text")
                    .property(Property::Input {
                        id: PropertyId::from("value"),
                        data_type: DataType::String,
                    })
                    .property(Property::Output {
                        id: PropertyId::from("return-value"),
                        data_type: DataType::String,
                    })
                    .build(),
            )
            .node(
                Node::builder("printer")
                    .property(Property::Command {
                        id: PropertyId::from("print"),
                    })
                    .property(Property::Input {
                        id: PropertyId::from("content"),
                        data_type: DataType::String,
                    })
                    .build(),
            )
            .build();

        assert_eq!(schema.nodes.len(), 3);
        let action = schema.nodes.get(&NodeId::from("action")).unwrap();
        assert_eq!(action.properties.len(), 1);
        assert!(action
            .properties
            .get(&PropertyId::from("executed"))
            .unwrap()
            .is_event());
        let text = schema.nodes.get(&NodeId::from("text")).unwrap();
        assert_eq!(text.properties.len(), 2);
        assert!(text
            .properties
            .get(&PropertyId::from("value"))
            .unwrap()
            .is_input());
        assert!(text
            .properties
            .get(&PropertyId::from("return-value"))
            .unwrap()
            .is_output());
        assert_eq!(
            *text
                .properties
                .get(&PropertyId::from("return-value"))
                .unwrap()
                .data_type()
                .unwrap(),
            DataType::String
        );
        let printer = schema.nodes.get(&NodeId::from("printer")).unwrap();
        assert_eq!(printer.properties.len(), 2);
        assert!(printer
            .properties
            .get(&PropertyId::from("print"))
            .unwrap()
            .is_command());
        assert!(printer
            .properties
            .get(&PropertyId::from("content"))
            .unwrap()
            .is_input());
    }

    #[test]
    #[should_panic]
    fn duplicate_node_id() {
        let _schema = Schema::builder()
            .node(Node::builder("a").build())
            .node(Node::builder("a").build())
            .build();
    }
}
