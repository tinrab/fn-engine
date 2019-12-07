//! Graph-based structure for representing computational flows.

use std::collections::HashMap;

use crate::error::GraphError;
use crate::graph::edge::{Edge, EdgeMap, Hook};
use crate::graph::placed_node::PlacedNode;
use crate::graph::property_value::PropertyValue;
use crate::schema::Schema;
use crate::value::Value;

pub mod edge;
pub mod placed_node;
pub mod property_value;

/// Represents a graph.
#[derive(Debug, Clone)]
pub struct Graph {
    /// Graph's nodes by key.
    pub nodes: HashMap<String, PlacedNode>,
    /// Graph's edges.
    pub edge_map: EdgeMap,
}

impl Graph {
    /// Constructs a `GraphBuilder`.
    pub fn builder(schema: &Schema) -> GraphBuilder {
        GraphBuilder::new(schema)
    }

    /// Returns a `PlacedNode` by key.
    /// # Panics
    /// If graph does not contains a node.
    pub fn get_node(&self, key: &str) -> &PlacedNode {
        self.nodes.get(key).unwrap()
    }
}

/// Utility for building graphs.
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

    /// Declares a new node.
    pub fn node(&mut self, id: &str, key: &str) -> Result<PlacedNode, GraphError> {
        if self.graph.nodes.contains_key(key) {
            return Err(GraphError::from(format!("Duplicate node key '{}'", id)));
        }
        let node = self.schema.nodes.get(id);
        if node.is_none() {
            return Err(GraphError::from(format!(
                "Node with id '{}' not found.",
                id
            )));
        }

        let placed_node = PlacedNode::new(node.unwrap(), key);
        self.graph.nodes.insert(key.into(), placed_node.clone());
        Ok(placed_node)
    }

    /// Assigns a value to a property.
    pub fn assign(
        &mut self,
        placed_node: &PlacedNode,
        property_id: &str,
        value: Value,
    ) -> Result<(), GraphError> {
        let property = placed_node.node.properties.get(property_id);
        if property.is_none() {
            return Err(GraphError::from(format!(
                "Node property '{}' not found for '{}'",
                property_id, placed_node.node.id
            )));
        }
        let property = property.unwrap().clone();
        let data_type = property.data_type();
        if data_type.is_none() {
            return Err(GraphError::new(
                "Can only assign values to data properties.",
            ));
        }
        if *data_type.unwrap() != value.data_type() {
            return Err(GraphError::new("Incompatible types."));
        }

        let mut values = placed_node.values.clone();
        values.insert(
            String::from(property_id),
            PropertyValue::new(property_id, value.clone()),
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

    /// Connects two properties by an edge.
    pub fn connect(
        &mut self,
        source_node: &PlacedNode,
        source_property_id: &str,
        target_node: &PlacedNode,
        target_property_id: &str,
    ) -> Result<(), GraphError> {
        let source_property = source_node.get_property(source_property_id);
        let target_property = target_node.get_property(target_property_id);

        if source_node.key == target_node.key {
            return Err(GraphError::new("Cannot connect to self."));
        }

        if !source_property.is_target() {
            return Err(GraphError::new("Invalid source property."));
        }
        if !target_property.is_source() {
            return Err(GraphError::new("Invalid target property."));
        }
        if source_property.is_event() && !target_property.is_command() {
            return Err(GraphError::new("Event can only be hooked to a command."));
        }
        if target_property.is_command() && !source_property.is_event() {
            return Err(GraphError::new(
                "Command can only be triggered by an event.",
            ));
        }
        if source_property.data_type() != target_property.data_type() {
            return Err(GraphError::new("Incompatible types."));
        }

        let edge = Edge::new(
            Hook::new(source_node.clone(), source_property.clone()),
            Hook::new(target_node.clone(), target_property.clone()),
        );

        if self.graph.edge_map.contains_edge(&edge) {
            return Err(GraphError::from(format!("Edge '{}' already exists.", edge)));
        }
        self.graph.edge_map.insert(&edge);

        Ok(())
    }

    /// Builds a `Graph`.
    pub fn build(self) -> Result<Graph, GraphError> {
        for placed_node in self.graph.nodes.values() {
            let missing_value_property = placed_node.node.properties.values().find(|property| {
                property.is_input() && !placed_node.values.contains_key(property.id())
            });

            if let Some(property) = missing_value_property {
                return Err(GraphError::from(format!(
                    "No value assigned for '{}#{}'",
                    placed_node.key,
                    property.id()
                )));
            }
        }

        Ok(self.graph.clone())
    }
}
