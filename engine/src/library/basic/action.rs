use graph::schema::node::Node;

pub const ID: &str = "action";
pub const COMMAND_TRIGGER: &str = "trigger";
pub const EVENT_TRIGGERED: &str = "triggered";

pub fn get() -> Node {
    Node::builder(&ID)
        .command(&COMMAND_TRIGGER)
        .event(&EVENT_TRIGGERED)
        .build()
}
