use std::collections::HashMap;

use crate::schema::node::{Node, NodeId};

pub mod node;
pub mod property;

#[derive(Debug, Clone)]
pub struct Schema {
    pub nodes: HashMap<NodeId, Node>,
}

pub struct SchemaBuilder {
    schema: Schema,
}

impl Schema {
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

    pub fn node(&'a mut self, node: Node) -> &'a mut SchemaBuilder {
        if let Some(node) = self.schema.nodes.insert(node.id.clone(), node) {
            panic!("duplicate node id '{}'", node.id);
        }
        self
    }

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
        let _schema = Schema::builder()
            .node(
                Node::builder("action")
                    .property(Property::Event {
                        id: PropertyId::from("execute"),
                    })
                    .build(),
            )
            .node(
                Node::builder("text")
                    .property(Property::Input {
                        id: PropertyId::from("value"),
                        data_type: DataType::String,
                        default_value: None,
                    })
                    .property(Property::Output {
                        id: PropertyId::from("return-value"),
                        data_type: DataType::String,
                        default_value: None,
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
                        default_value: None,
                    })
                    .build(),
            )
            .build();
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
