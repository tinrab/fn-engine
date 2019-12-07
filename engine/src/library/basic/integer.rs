use graph::schema::node::Node;
use graph::value::DataType;

pub const ID: &str = "integer";
pub const INPUT_VALUE: &str = "value";
pub const OUTPUT_RETURN_VALUE: &str = "return-value";

pub fn get() -> Node {
    Node::builder(&ID)
        .input(&INPUT_VALUE, DataType::Integer)
        .output(&OUTPUT_RETURN_VALUE, DataType::Integer)
        .build()
}
