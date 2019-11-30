//! Value assigned to a property.

use crate::schema::property::PropertyId;
use crate::value::Value;

/// Represents a value assigned to a property.
#[derive(Debug, Clone)]
pub struct PropertyValue {
    /// Id of a property.
    pub property_id: PropertyId,
    /// Assigned value.
    pub value: Value,
}

impl PropertyValue {
    /// Constructs a `PropertyValue`.
    pub fn new(property_id: PropertyId, value: Value) -> Self {
        PropertyValue { property_id, value }
    }
}
