use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use crate::graph::placed_node::PlacedNode;
use crate::schema::property::Property;

#[derive(Debug, Clone)]
pub struct Hook {
    pub node: PlacedNode,
    pub property: Property,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: Hook,
    pub target: Hook,
}

#[derive(Debug, Clone)]
pub struct EdgeMap {
    pub edges: HashMap<String, Edge>,
    inputs: HashMap<String, Hook>,
    outputs: HashMap<String, Vec<Hook>>,
}

impl Hook {
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
    pub fn contains_edge(&self, edge: &Edge) -> bool {
        self.contains_hooks(&edge.source, &edge.target)
    }

    pub fn contains_hooks(&self, source: &Hook, target: &Hook) -> bool {
        self.contains_edge_between(
            &source.node,
            &source.property,
            &target.node,
            &target.property,
        )
    }

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

    pub fn insert(&mut self, edge: &Edge) -> Option<Edge> {
        self.edges.insert(edge.to_string(), edge.clone())
    }

    pub fn get_input(&self, target: &Hook) -> Option<Hook> {
        self.inputs.get(target.to_string().as_str()).cloned()
    }

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
