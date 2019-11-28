#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
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
