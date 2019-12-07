//! Schema describes what elements can graphs contain.

use std::collections::HashMap;

use crate::schema::node::Node;

pub mod node;
pub mod property;

/// Holds available elements for building graphs.
#[derive(Debug, Clone)]
pub struct Schema {
    /// All available nodes.
    pub nodes: HashMap<String, Node>,
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
