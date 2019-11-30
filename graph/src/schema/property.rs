use std::fmt::{Display, Error, Formatter};

use crate::value::{DataType, Value};

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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Property {
    Event {
        id: PropertyId,
    },
    Command {
        id: PropertyId,
    },
    Input {
        id: PropertyId,
        data_type: DataType,
        default_value: Option<Value>,
    },
    Output {
        id: PropertyId,
        data_type: DataType,
        default_value: Option<Value>,
    },
}

impl Property {
    pub fn id(&self) -> &PropertyId {
        match self {
            Property::Event { id } => id,
            Property::Command { id } => id,
            Property::Input { id, .. } => id,
            Property::Output { id, .. } => id,
        }
    }

    pub fn is_data(&self) -> bool {
        self.data_type().is_some()
    }

    pub fn is_control(&self) -> bool {
        !self.is_data()
    }

    pub fn is_event(&self) -> bool {
        if let Property::Event { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_command(&self) -> bool {
        if let Property::Command { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_input(&self) -> bool {
        if let Property::Input { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_output(&self) -> bool {
        if let Property::Output { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn is_source(&self) -> bool {
        match self {
            Property::Input { .. } => true,
            Property::Command { .. } => true,
            _ => false,
        }
    }

    pub fn is_target(&self) -> bool {
        !self.is_source()
    }

    pub fn data_type(&self) -> Option<&DataType> {
        match self {
            Property::Input { data_type, .. } => Some(data_type),
            Property::Output { data_type, .. } => Some(data_type),
            _ => None,
        }
    }
}
