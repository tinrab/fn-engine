use crate::schema::property::Property;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct PropertyValue {
    pub property: Property,
    pub value: Value,
}

impl PropertyValue {
    pub fn new(property: Property, value: Value) -> Self {
        PropertyValue { property, value }
    }
}
