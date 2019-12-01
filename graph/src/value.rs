//! Base constructs for working with values.

/// Value's data type.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum DataType {
    /// Type for `i64`.
    Integer,
    /// Type for `f64`.
    Float,
    /// Type for `bool`.
    Boolean,
    /// Type for `String`.
    String,
}

/// Value type.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    /// Holds a `i64`.
    Integer(i64),
    /// Holds a `f64`.
    Float(f64),
    /// Holds a `bool`.
    Boolean(bool),
    /// Holds a `String`.
    String(String),
}

impl Value {
    /// Returns default value for type.
    pub fn default_for(data_type: DataType) -> Self {
        match data_type {
            DataType::Integer => Value::from(0),
            DataType::Float => Value::from(0.0),
            DataType::Boolean => Value::from(false),
            DataType::String => Value::from(""),
        }
    }

    /// Returns Value's data type.
    pub fn data_type(&self) -> DataType {
        match self {
            Value::Integer(_) => DataType::Integer,
            Value::Float(_) => DataType::Float,
            Value::Boolean(_) => DataType::Boolean,
            Value::String(_) => DataType::String,
        }
    }
}

impl From<i64> for Value {
    fn from(a: i64) -> Self {
        Value::Integer(a)
    }
}

impl From<f64> for Value {
    fn from(a: f64) -> Self {
        Value::Float(a)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(String::from(s))
    }
}
