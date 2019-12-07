use graph::schema::node::Node;
use graph::value::DataType;

pub const ID: &str = "repeat";
pub const COMMAND_START: &str = "start";
pub const EVENT_EXECUTED: &str = "executed";
pub const INPUT_TIMES: &str = "times";

pub fn get() -> Node {
    Node::builder(&ID)
        .command(&COMMAND_START)
        .event(&EVENT_EXECUTED)
        .input(&INPUT_TIMES, DataType::Integer)
        .build()
}
