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

    pub fn get_node(&self, key: &str) -> &PlacedNode {
        self.nodes.get(key).unwrap()
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
        let source_property = source_node.get_property(&source_property_id);
        let target_property = target_node.get_property(&target_property_id);

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
            Hook::new(source_node.clone(), source_property.clone()),
            Hook::new(target_node.clone(), target_property.clone()),
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
