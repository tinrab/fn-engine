//! Properties declared for nodes.

use std::fmt::{Display, Error, Formatter};

use crate::value::{DataType, Value};

/// Type for property ids.
#[derive(Debug, Hash, Clone, PartialOrd, Eq, PartialEq)]
pub struct PropertyId(String);

impl Display for PropertyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.0.as_str())
    }
}

impl From<&str> for PropertyId {
    fn from(s: &str) -> Self {
        PropertyId(String::from(s))
    }
}

impl Into<String> for PropertyId {
    fn into(self) -> String {
        self.0
    }
}

/// Represents a property.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Property {
    /// Event can trigger a command.
    Event {
        /// Property's id.
        id: PropertyId,
    },
    /// Command executes specific computation inside a node.
    Command {
        /// Property's id.
        id: PropertyId,
    },
    /// Input data for a node.
    Input {
        /// Property's id.
        id: PropertyId,
        /// Property's data type.
        data_type: DataType,
        /// Property's optional default value. Must be assigned if `None`.
        default_value: Option<Value>,
    },
    /// Output is produces by a node.
    Output {
        /// Property's id.
        id: PropertyId,
        /// Property's data type.
        data_type: DataType,
    },
}

impl Property {
    /// Returns property's id.
    pub fn id(&self) -> &PropertyId {
        match self {
            Property::Event { id } => id,
            Property::Command { id } => id,
            Property::Input { id, .. } => id,
            Property::Output { id, .. } => id,
        }
    }

    /// Returns whether property is `Property::Input` or `Property::Output`.
    pub fn is_data(&self) -> bool {
        self.data_type().is_some()
    }

    /// Returns whether property is `Property::Command` or `Property::Event`.
    pub fn is_control(&self) -> bool {
        !self.is_data()
    }

    /// Returns whether property is `Property::Event`.
    pub fn is_event(&self) -> bool {
        if let Property::Event { .. } = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property is `Property::Command`.
    pub fn is_command(&self) -> bool {
        if let Property::Command { .. } = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property is `Property::Input`.
    pub fn is_input(&self) -> bool {
        if let Property::Input { .. } = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property is `Property::Output`.
    pub fn is_output(&self) -> bool {
        if let Property::Output { .. } = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property can be used as a source.
    pub fn is_source(&self) -> bool {
        match self {
            Property::Input { .. } => true,
            Property::Command { .. } => true,
            _ => false,
        }
    }

    /// Returns whether property can be used as a target.
    pub fn is_target(&self) -> bool {
        !self.is_source()
    }

    /// Returns property's data type if property is an input or output.
    pub fn data_type(&self) -> Option<&DataType> {
        match self {
            Property::Input { data_type, .. } => Some(data_type),
            Property::Output { data_type, .. } => Some(data_type),
            _ => None,
        }
    }
}
