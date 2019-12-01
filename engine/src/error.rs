//! Error type to be used inside `engine` module.

use std::error::Error as StdError;
use std::fmt::{Display, Error, Formatter};

use config::ConfigError;

use graph::error::GraphError;

/// Error representing an error with an engine.
#[derive(Debug)]
pub struct EngineError {
    /// Error message.
    pub message: String,
}

impl EngineError {
    /// Constructs a new `EngineError`.
    pub fn new(message: &str) -> Self {
        EngineError {
            message: String::from(message),
        }
    }
}

impl From<String> for EngineError {
    fn from(s: String) -> Self {
        EngineError { message: s }
    }
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "GraphError: {:?}", self.message)
    }
}

impl StdError for EngineError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl From<GraphError> for EngineError {
    fn from(e: GraphError) -> Self {
        EngineError { message: e.message }
    }
}

impl From<ConfigError> for EngineError {
    fn from(e: ConfigError) -> Self {
        EngineError {
            message: String::from(e.description()),
        }
    }
}
