use graph::schema::node::Node;
use graph::value::DataType;

pub const ID: &str = "printer";
pub const COMMAND_PRINT: &str = "print";
pub const INPUT_CONTENT: &str = "content";

pub fn get() -> Node {
    Node::builder(&ID)
        .command(&COMMAND_PRINT)
        .input(&INPUT_CONTENT, DataType::Integer)
        .build()
}
