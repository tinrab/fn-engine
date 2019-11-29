use std::collections::HashMap;

use crate::node::{Node, NodeId};

#[derive(Debug, Clone)]
pub struct Schema {
    nodes: HashMap<NodeId, Node>,
}

pub struct SchemaBuilder {
    schema: Schema,
}

impl<'a> Schema {
    pub fn builder() -> SchemaBuilder {
        SchemaBuilder::new()
    }
}

impl<'a> SchemaBuilder {
    pub fn new() -> Self {
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
    use crate::node::{Node, NodeId};
    use crate::property::{Property, PropertyId};
    use crate::schema::Schema;
    use crate::value::DataType;

    #[test]
    fn basic() {
        let _schema = Schema::builder()
            .node(
                Node::new(NodeId::from("action"))
                    .property(Property::Event {
                        id: PropertyId::from("execute"),
                    })
                    .clone(),
            )
            .node(
                Node::new(NodeId::from("text"))
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
                    .clone(),
            )
            .node(
                Node::new(NodeId::from("printer"))
                    .property(Property::Command {
                        id: PropertyId::from("print"),
                    })
                    .property(Property::Input {
                        id: PropertyId::from("content"),
                        data_type: DataType::String,
                        default_value: None,
                    })
                    .clone(),
            )
            .build();
    }

    #[test]
    #[should_panic]
    fn duplicate_node_id() {
        let _schema = Schema::builder()
            .node(Node::new(NodeId::from("a")).clone())
            .node(Node::new(NodeId::from("a")).clone())
            .build();
    }
}
