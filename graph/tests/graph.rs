use std::panic::catch_unwind;

use graph::graph::Graph;
use graph::schema::node::Node;
use graph::schema::Schema;
use graph::value::{DataType, Value};

const NODE_A: &str = "a";
const NODE_B: &str = "b";
const NODE_C: &str = "c";
const INPUT_STRING: &str = "input-string";
const INPUT_INTEGER: &str = "input-integer";
const OUTPUT_INTEGER: &str = "output-integer";
const OUTPUT_STRING: &str = "output-string";
const EVENT: &str = "event";
const COMMAND: &str = "command";

#[test]
fn build_graph() {
    let schema = build_schema();
    let graph = {
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node(NODE_A, "a1").unwrap();
        let b1 = graph_builder.node(NODE_B, "b1").unwrap();
        let c1 = graph_builder.node(NODE_C, "c1").unwrap();

        graph_builder
            .assign(&a1, &INPUT_STRING, Value::from("abc"))
            .unwrap();
        graph_builder
            .assign(&b1, &INPUT_INTEGER, Value::from(1))
            .unwrap();
        graph_builder
            .assign(&c1, &INPUT_INTEGER, Value::from(2))
            .unwrap();

        graph_builder.connect(&a1, &EVENT, &b1, &COMMAND).unwrap();
        graph_builder.connect(&b1, &EVENT, &c1, &COMMAND).unwrap();
        graph_builder
            .connect(&b1, &OUTPUT_INTEGER, &c1, &INPUT_INTEGER)
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
        a1.get_property(&EVENT),
        b1,
        b1.get_property(&COMMAND),
    ));
    assert!(graph.edge_map.contains_edge_between(
        b1,
        b1.get_property(&EVENT),
        c1,
        c1.get_property(&COMMAND),
    ));
    assert!(graph.edge_map.contains_edge_between(
        b1,
        b1.get_property(&OUTPUT_INTEGER),
        c1,
        c1.get_property(&INPUT_INTEGER),
    ));
}

#[test]
fn errors() {
    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        graph_builder.node(&NODE_A, "a1").unwrap();
        graph_builder.node(&NODE_A, "a1").unwrap();
    })
    .is_err());

    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node(&NODE_A, "a1").unwrap();
        graph_builder
            .assign(&a1, &COMMAND, Value::Integer(42))
            .unwrap();
    })
    .is_err());

    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node(&NODE_A, "a1").unwrap();
        let a2 = graph_builder.node(&NODE_A, "a1").unwrap();
        graph_builder.connect(&a1, &EVENT, &a2, &COMMAND).unwrap();
    })
    .is_err());
}

fn build_schema() -> Schema {
    Schema::builder()
        .node(
            Node::builder(&NODE_A)
                .event(&EVENT)
                .command(&COMMAND)
                .input(&INPUT_STRING, DataType::String)
                .output(&OUTPUT_STRING, DataType::String)
                .build(),
        )
        .node(
            Node::builder(&NODE_B)
                .event(&EVENT)
                .command(&COMMAND)
                .input(&INPUT_INTEGER, DataType::Integer)
                .output(&OUTPUT_INTEGER, DataType::Integer)
                .build(),
        )
        .node(
            Node::builder(&NODE_C)
                .event(&EVENT)
                .command(&COMMAND)
                .input(&INPUT_INTEGER, DataType::Integer)
                .output(&OUTPUT_INTEGER, DataType::Integer)
                .build(),
        )
        .build()
}
