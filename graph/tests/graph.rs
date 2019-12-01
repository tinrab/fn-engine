use std::panic::catch_unwind;

use graph::graph::Graph;
use graph::schema::node::Node;
use graph::schema::Schema;
use graph::value::{DataType, Value};

#[test]
fn build_graph() {
    let schema = build_schema();
    let graph = {
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node("a", "a1").unwrap();
        let b1 = graph_builder.node("b", "b1").unwrap();
        let c1 = graph_builder.node("c", "c1").unwrap();

        graph_builder
            .assign(&a1, "input-string", Value::from("abc"))
            .unwrap();
        graph_builder
            .assign(&b1, "input-integer", Value::from(1))
            .unwrap();
        graph_builder
            .assign(&c1, "input-integer", Value::from(2))
            .unwrap();

        graph_builder.connect(&a1, "event", &b1, "command").unwrap();
        graph_builder.connect(&b1, "event", &c1, "command").unwrap();
        graph_builder
            .connect(&b1, "output-integer", &c1, "input-integer")
            .unwrap();

        graph_builder.build().unwrap()
    };

    assert_eq!(graph.nodes.len(), 3);
    assert_eq!(graph.edge_map.edges.len(), 3);
    let a1 = graph.get_node("a1");
    let b1 = graph.get_node("b1");
    let c1 = graph.get_node("c1");
    assert!(graph.edge_map.contains_edge_between(
        a1,
        a1.get_property_by_id("event"),
        b1,
        b1.get_property_by_id("command"),
    ));
    assert!(graph.edge_map.contains_edge_between(
        b1,
        b1.get_property_by_id("event"),
        c1,
        c1.get_property_by_id("command"),
    ));
    assert!(graph.edge_map.contains_edge_between(
        b1,
        b1.get_property_by_id("output-integer"),
        c1,
        c1.get_property_by_id("input-integer"),
    ));
}

#[test]
fn errors() {
    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        graph_builder.node("a", "a1").unwrap();
        graph_builder.node("a", "a1").unwrap();
    })
    .is_err());

    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node("a", "a1").unwrap();
        graph_builder
            .assign(&a1, "command", Value::Integer(42))
            .unwrap();
    })
    .is_err());

    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node("a", "a1").unwrap();
        let a2 = graph_builder.node("a", "a1").unwrap();
        graph_builder.connect(&a1, "event", &a2, "command").unwrap();
    })
    .is_err());
}

fn build_schema() -> Schema {
    Schema::builder()
        .node(
            Node::builder("a")
                .event("event")
                .command("command")
                .input("input-string", DataType::String)
                .output("output-string", DataType::String)
                .build(),
        )
        .node(
            Node::builder("b")
                .event("event")
                .command("command")
                .input("input-integer", DataType::Integer)
                .output("output-integer", DataType::Integer)
                .build(),
        )
        .node(
            Node::builder("c")
                .event("event")
                .command("command")
                .input("input-integer", DataType::Integer)
                .output("output-integer", DataType::Integer)
                .build(),
        )
        .build()
}
