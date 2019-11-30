#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

//! Engine for executing graph based programs.

use graph::graph::Graph;
use graph::schema::Schema;

pub mod error;
pub mod schema;

/// Used to coordinate executing of graphs.
pub struct Engine {
    /// Schema used by this engine.
    pub schema: Schema,
}

impl Engine {
    /// Constructs a new engine for executing graphs based on `schema`.
    pub fn new(schema: Schema) -> Self {
        Engine { schema }
    }

    /// Run the engine.
    pub fn run(&self) {}

    /// Execute a graph on this engine. An engine must be ran first.
    pub fn execute(&self, _graph: &Graph) {}
}
