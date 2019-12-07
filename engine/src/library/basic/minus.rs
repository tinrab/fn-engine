use graph::schema::node::Node;
use graph::value::DataType;

pub const ID: &str = "minus";
pub const INPUT_A: &str = "a";
pub const INPUT_B: &str = "b";
pub const OUTPUT_C: &str = "c";

pub fn get() -> Node {
    Node::builder(&ID)
        .input(&INPUT_A, DataType::Integer)
        .input(&INPUT_B, DataType::Integer)
        .output(&OUTPUT_C, DataType::Integer)
        .build()
}
