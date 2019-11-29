use std::collections::HashMap;

use crate::error::GraphError;
use crate::graph::edge::EdgeMap;
use crate::graph::placed_node::PlacedNode;
use crate::graph::property_value::PropertyValue;
use crate::schema::node::NodeId;
use crate::schema::property::PropertyId;
use crate::schema::Schema;
use crate::value::Value;

pub mod edge;
pub mod placed_node;
pub mod property_value;

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: HashMap<String, PlacedNode>,
    edge_map: EdgeMap,
}

impl Graph {
    pub fn builder(schema: &Schema) -> GraphBuilder {
        GraphBuilder::new(schema)
    }
}

pub struct GraphBuilder<'a> {
    schema: &'a Schema,
    graph: Graph,
}

impl<'a> GraphBuilder<'a> {
    fn new(schema: &'a Schema) -> Self {
        GraphBuilder {
            schema,
            graph: Graph {
                nodes: Default::default(),
                edge_map: EdgeMap::default(),
            },
        }
    }

    pub fn node(&mut self, id: &str, key: &str) -> Result<PlacedNode, GraphError> {
        if self.graph.nodes.contains_key(key) {
            return Err(GraphError::GraphBuilder(format!(
                "Duplicate node key '{}'",
                id
            )));
        }
        let node = self.schema.nodes.get(&NodeId::from(id));
        if node.is_none() {
            return Err(GraphError::GraphBuilder(format!(
                "Node with id '{}' not found.",
                id
            )));
        }

        let placed_node = PlacedNode::new(node.unwrap(), key);
        self.graph.nodes.insert(key.into(), placed_node.clone());
        Ok(placed_node)
    }

    pub fn assign(
        &mut self,
        placed_node: PlacedNode,
        property_id: PropertyId,
        value: Value,
    ) -> Result<(), GraphError> {
        let property = placed_node.node.properties.get(&property_id);
        if property.is_none() {
            return Err(GraphError::GraphBuilder(format!(
                "Node property '{}' not found for '{}'",
                property_id, placed_node.node.id
            )));
        }
        let property = property.unwrap().clone();
        let data_type = property.data_type();
        if data_type.is_none() {
            return Err(GraphError::GraphBuilder(String::from(
                "Can only assign values to data properties.",
            )));
        }
        if data_type.unwrap().clone() != value.data_type() {
            return Err(GraphError::GraphBuilder(String::from(
                "Incompatible types.",
            )));
        }

        let mut values = placed_node.values.clone();
        values.insert(
            property_id.clone(),
            PropertyValue::new(property, value.clone()),
        );
        self.graph.nodes.insert(
            placed_node.key.clone(),
            PlacedNode {
                values,
                ..placed_node.clone()
            },
        );

        Ok(())
    }
    pub fn build(self) -> Graph {
        self.graph.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use crate::schema::node::Node;
    use crate::schema::property::{Property, PropertyId};
    use crate::schema::Schema;
    use crate::value::{DataType, Value};

    #[test]
    fn basic() {
        let schema = build_schema();
        let graph = {
            let mut graph_builder = Graph::builder(&schema);
            let _a1 = graph_builder.node("a", "a1").unwrap();
            let b1 = graph_builder.node("b", "b1").unwrap();
            graph_builder
                .assign(b1, PropertyId::from("input-integer"), Value::Integer(42))
                .unwrap();
            let _c1 = graph_builder.node("c", "c1").unwrap();
            graph_builder.build()
        };

        println!("{:#?}", graph);
    }

    fn build_schema() -> Schema {
        Schema::builder()
            .node(
                Node::builder("a")
                    .property(Property::Event {
                        id: PropertyId::from("event"),
                    })
                    .property(Property::Command {
                        id: PropertyId::from("command"),
                    })
                    .property(Property::Input {
                        id: PropertyId::from("input-string"),
                        data_type: DataType::String,
                        default_value: None,
                    })
                    .property(Property::Output {
                        id: PropertyId::from("output-string"),
                        data_type: DataType::String,
                        default_value: None,
                    })
                    .build(),
            )
            .node(
                Node::builder("b")
                    .property(Property::Event {
                        id: PropertyId::from("event"),
                    })
                    .property(Property::Command {
                        id: PropertyId::from("command"),
                    })
                    .property(Property::Input {
                        id: PropertyId::from("input-integer"),
                        data_type: DataType::Integer,
                        default_value: None,
                    })
                    .property(Property::Output {
                        id: PropertyId::from("output-integer"),
                        data_type: DataType::Integer,
                        default_value: None,
                    })
                    .build(),
            )
            .node(
                Node::builder("c")
                    .property(Property::Event {
                        id: PropertyId::from("event"),
                    })
                    .property(Property::Command {
                        id: PropertyId::from("command"),
                    })
                    .property(Property::Input {
                        id: PropertyId::from("input-integer"),
                        data_type: DataType::Integer,
                        default_value: None,
                    })
                    .property(Property::Output {
                        id: PropertyId::from("output-integer"),
                        data_type: DataType::Integer,
                        default_value: None,
                    })
                    .build(),
            )
            .build()
    }
}
