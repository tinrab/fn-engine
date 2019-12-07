//! Value assigned to a property.

use crate::value::Value;

/// Represents a value assigned to a property.
#[derive(Debug, Clone)]
pub struct PropertyValue {
    /// Id of a property.
    pub property_id: String,
    /// Assigned value.
    pub value: Value,
}

impl PropertyValue {
    /// Constructs a `PropertyValue`.
    pub fn new(property_id: &str, value: Value) -> Self {
        PropertyValue {
            property_id: String::from(property_id),
            value,
        }
    }
}
