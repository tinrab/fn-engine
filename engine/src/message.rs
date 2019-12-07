//! Types for inter-worker messaging.

use std::sync::Arc;

use graph::graph::placed_node::PlacedNode;
use graph::graph::Graph;

/// Context for current point of execution.
#[derive(Debug)]
pub struct Context {
    pub graph: Arc<Graph>,
    pub node: PlacedNode,
}

/// Represents a message that triggers a command.
#[derive(Debug)]
pub struct Instruction {
    pub context: Context,
    pub command_id: String,
}

/// Message represents a type for communication between workers.
#[derive(Debug)]
pub enum Message {
    Instruction(Instruction),
}

impl Context {
    /// Constructs a `Context`.
    pub fn new(graph: Arc<Graph>, node: &PlacedNode) -> Self {
        Context {
            graph,
            node: node.clone(),
        }
    }
}

impl Message {
    /// Constructs `Message::Instruction`.
    pub fn instruction(context: Context, command_id: &str) -> Self {
        Message::Instruction(Instruction {
            context,
            command_id: String::from(command_id),
        })
    }
}
