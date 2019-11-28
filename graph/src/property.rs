use crate::value::{DataType, Value};

#[derive(Debug, Hash, Clone, PartialOrd, Eq, PartialEq)]
pub struct PropertyId(String);

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
