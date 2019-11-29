use std::collections::HashMap;

use crate::error::GraphError;
use crate::graph::edge::{Edge, EdgeMap, Hook};
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
    pub nodes: HashMap<String, PlacedNode>,
    pub edge_map: EdgeMap,
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
        placed_node: &PlacedNode,
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

    pub fn connect(
        &mut self,
        source_node: &PlacedNode,
        source_property_id: PropertyId,
        target_node: &PlacedNode,
        target_property_id: PropertyId,
    ) -> Result<(), GraphError> {
        let source_property = source_node.node.properties.get(&source_property_id);
        if source_property.is_none() {
            return Err(GraphError::GraphBuilder(format!(
                "Node property '{}' not found for '{}'",
                source_property_id, source_node.node.id
            )));
        }
        let source_property = source_property.unwrap().clone();

        let target_property = target_node.node.properties.get(&target_property_id);
        if target_property.is_none() {
            return Err(GraphError::GraphBuilder(format!(
                "Node property '{}' not found for '{}'",
                target_property_id, target_node.node.id
            )));
        }
        let target_property = target_property.unwrap().clone();

        if source_node.key == target_node.key {
            return Err(GraphError::GraphBuilder(String::from(
                "Cannot connect to self.",
            )));
        }

        if !source_property.is_target() {
            return Err(GraphError::GraphBuilder(String::from(
                "Invalid source property.",
            )));
        }
        if !target_property.is_source() {
            return Err(GraphError::GraphBuilder(String::from(
                "Invalid target property.",
            )));
        }
        if source_property.is_event() && !target_property.is_command() {
            return Err(GraphError::GraphBuilder(String::from(
                "Event can only be hooked to a command.",
            )));
        }
        if target_property.is_command() && !source_property.is_event() {
            return Err(GraphError::GraphBuilder(String::from(
                "Command can only be triggered by an event.",
            )));
        }
        if source_property.data_type() != target_property.data_type() {
            return Err(GraphError::GraphBuilder(String::from(
                "Incompatible types.",
            )));
        }

        let edge = Edge::new(
            Hook::new(source_node.clone(), source_property),
            Hook::new(target_node.clone(), target_property),
        );

        if self.graph.edge_map.contains_edge(&edge) {
            return Err(GraphError::GraphBuilder(format!(
                "Edge '{}' already exists.",
                edge
            )));
        }
        self.graph.edge_map.insert(&edge);

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
            let a1 = graph_builder.node("a", "a1").unwrap();
            let b1 = graph_builder.node("b", "b1").unwrap();
            graph_builder
                .assign(&b1, PropertyId::from("input-integer"), Value::Integer(42))
                .unwrap();
            let c1 = graph_builder.node("c", "c1").unwrap();

            graph_builder
                .connect(
                    &a1,
                    PropertyId::from("event"),
                    &b1,
                    PropertyId::from("command"),
                )
                .unwrap();
            graph_builder
                .connect(
                    &b1,
                    PropertyId::from("event"),
                    &c1,
                    PropertyId::from("command"),
                )
                .unwrap();

            graph_builder.build()
        };

        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edge_map.edges.len(), 2);
        let a1 = graph.nodes.get("a1").unwrap();
        let b1 = graph.nodes.get("b1").unwrap();
        let c1 = graph.nodes.get("c1").unwrap();
        assert!(graph.edge_map.contains_edge_between(
            a1,
            a1.node.properties.get(&PropertyId::from("event")).unwrap(),
            b1,
            b1.node
                .properties
                .get(&PropertyId::from("command"))
                .unwrap(),
        ));
        assert!(graph.edge_map.contains_edge_between(
            b1,
            b1.node.properties.get(&PropertyId::from("event")).unwrap(),
            c1,
            c1.node
                .properties
                .get(&PropertyId::from("command"))
                .unwrap(),
        ));
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
