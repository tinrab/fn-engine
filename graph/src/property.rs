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
            Property::Command { id } => id,
            Property::Event { id } => id,
            Property::Input {
                id,
                data_type: _,
                default_value: _,
            } => id,
            Property::Output {
                id,
                data_type: _,
                default_value: _,
            } => id,
        }
    }
}
