//! Contains up-to-date schema.

use graph::schema::node::Node;
use graph::schema::Schema;
use graph::value::DataType;

/// Returns a `Schema` supported by current implementation of the engine.
pub fn get() -> Schema {
    Schema::builder()
        .node(
            Node::builder("action")
                .command("trigger")
                .event("triggered")
                .build(),
        )
        .node(
            Node::builder("repeat")
                .command("start")
                .event("executed")
                .input("times", DataType::Integer)
                .build(),
        )
        .node(
            Node::builder("printer")
                .command("print")
                .input("content", DataType::Integer)
                .build(),
        )
        .node(
            Node::builder("plus")
                .input("a", DataType::Integer)
                .input("b", DataType::Integer)
                .output("c", DataType::Integer)
                .build(),
        )
        .node(
            Node::builder("minus")
                .input("a", DataType::Integer)
                .input("b", DataType::Integer)
                .output("c", DataType::Integer)
                .build(),
        )
        .node(
            Node::builder("integer")
                .input("value", DataType::Integer)
                .output("return-value", DataType::Integer)
                .build(),
        )
        .build()
}
