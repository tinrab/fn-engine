//! Edge connects to node's properties.

use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use crate::graph::placed_node::PlacedNode;
use crate::schema::property::Property;

/// Represents a point of connection on a node.
#[derive(Debug, Clone)]
pub struct Hook {
    /// Target node.
    pub node: PlacedNode,
    /// Target node's property.
    pub property: Property,
}

/// Represents a connection between two properties.
#[derive(Debug, Clone)]
pub struct Edge {
    /// Source hook.
    pub source: Hook,
    /// Target hook.
    pub target: Hook,
}

/// Contains edges of a graph.
#[derive(Debug, Clone)]
pub struct EdgeMap {
    /// All edges by generated key.
    pub edges: HashMap<String, Edge>,
    inputs: HashMap<String, Hook>,
    outputs: HashMap<String, Vec<Hook>>,
}

impl Hook {
    /// Constructs a `Hook`.
    pub fn new(node: PlacedNode, property: Property) -> Self {
        Hook { node, property }
    }
}

impl Display for Hook {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}#{}", self.node.key, self.property.id())
    }
}

impl Edge {
    /// Constructs an `Edge`.
    pub fn new(source: Hook, target: Hook) -> Self {
        Edge { source, target }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}>{}", self.source, self.target)
    }
}

impl EdgeMap {
    /// Returns whether an edge exists.
    pub fn contains_edge(&self, edge: &Edge) -> bool {
        self.contains_hooks(&edge.source, &edge.target)
    }

    /// Returns whether an edge exists between hooks.
    pub fn contains_hooks(&self, source: &Hook, target: &Hook) -> bool {
        self.contains_edge_between(
            &source.node,
            &source.property,
            &target.node,
            &target.property,
        )
    }

    /// Returns whether an edge exists between properties.
    pub fn contains_edge_between(
        &self,
        source_node: &PlacedNode,
        source_property: &Property,
        target_node: &PlacedNode,
        target_property: &Property,
    ) -> bool {
        self.edges.contains_key(
            EdgeMap::get_edge_key(source_node, source_property, target_node, target_property)
                .as_str(),
        )
    }

    /// Inserts an edge. Returns old value if it existed.
    pub fn insert(&mut self, edge: &Edge) -> Option<Edge> {
        self.edges.insert(edge.to_string(), edge.clone())
    }

    /// Returns an input hook for a target if it exists.
    pub fn get_input(&self, target: &Hook) -> Option<Hook> {
        self.inputs.get(target.to_string().as_str()).cloned()
    }

    /// Returns all output hooks for a source.
    pub fn get_outputs(&self, source: &Hook) -> Vec<Hook> {
        self.outputs
            .get(source.to_string().as_str())
            .cloned()
            .unwrap_or_default()
    }

    fn get_edge_key(
        source_node: &PlacedNode,
        source_property: &Property,
        target_node: &PlacedNode,
        target_property: &Property,
    ) -> String {
        format!(
            "{}#{}>{}#{}",
            source_node.key,
            source_property.id(),
            target_node.key,
            target_property.id()
        )
    }
}

impl Default for EdgeMap {
    fn default() -> Self {
        EdgeMap {
            edges: Default::default(),
            inputs: Default::default(),
            outputs: Default::default(),
        }
    }
}
