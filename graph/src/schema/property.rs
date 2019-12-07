//! Properties declared for nodes.

use crate::value::DataType;

/// Event can trigger a command.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct EventProperty {
    /// Property's id.
    pub id: String,
}

/// Command executes specific computation inside a node.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CommandProperty {
    /// Property's id.
    pub id: String,
}

/// Input data for a node.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct InputProperty {
    /// Property's id.
    pub id: String,
    /// Property's data type.
    pub data_type: DataType,
}

/// Output is produces by a node.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OutputProperty {
    /// Property's id.
    pub id: String,
    /// Property's data type.
    pub data_type: DataType,
}

/// Represents a property.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Property {
    /// Event can trigger a command.
    Event(EventProperty),
    /// Command executes specific computation inside a node.
    Command(CommandProperty),
    /// Input data for a node.
    Input(InputProperty),
    /// Output is produces by a node.
    Output(OutputProperty),
}

impl Property {
    /// Returns property's id.
    pub fn id(&self) -> &String {
        match self {
            Property::Event(property) => &property.id,
            Property::Command(property) => &property.id,
            Property::Input(property) => &property.id,
            Property::Output(property) => &property.id,
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
        if let Property::Event(_) = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property is `Property::Command`.
    pub fn is_command(&self) -> bool {
        if let Property::Command(_) = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property is `Property::Input`.
    pub fn is_input(&self) -> bool {
        if let Property::Input(_) = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property is `Property::Output`.
    pub fn is_output(&self) -> bool {
        if let Property::Output(_) = self {
            true
        } else {
            false
        }
    }

    /// Returns whether property can be used as a source.
    pub fn is_source(&self) -> bool {
        match self {
            Property::Input(_) => true,
            Property::Command(_) => true,
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
            Property::Input(property) => Some(&property.data_type),
            Property::Output(property) => Some(&property.data_type),
            _ => None,
        }
    }
}
