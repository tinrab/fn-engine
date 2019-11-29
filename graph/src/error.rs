use std::error;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum GraphError {
    Unknown,
    GraphBuilder(String),
}

impl Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl<'a> error::Error for GraphError {
    fn description(&self) -> &str {
        match self {
            GraphError::GraphBuilder(e) => e,
            GraphError::Unknown => "Unknown error.",
        }
    }
}
