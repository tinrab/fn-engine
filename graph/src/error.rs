//! Error type to be used inside `graph` module.

use std::error;
use std::fmt::{Display, Error, Formatter};

/// Error representing an error with a graph.
#[derive(Debug)]
pub struct GraphError {
    /// Error message.
    pub message: String,
}

impl GraphError {
    /// Constructs a new `GraphError`.
    pub fn new(message: &str) -> Self {
        GraphError {
            message: String::from(message),
        }
    }
}

impl From<String> for GraphError {
    fn from(s: String) -> Self {
        GraphError { message: s }
    }
}

impl Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "GraphError: {:?}", self.message)
    }
}

impl<'a> error::Error for GraphError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}
