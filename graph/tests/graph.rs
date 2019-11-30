use std::panic::catch_unwind;

use graph::graph::Graph;
use graph::schema::node::Node;
use graph::schema::property::{Property, PropertyId};
use graph::schema::Schema;
use graph::value::{DataType, Value};

#[test]
fn build_graph() {
    let schema = build_schema();
    let graph = {
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node("a", "a1").unwrap();
        let b1 = graph_builder.node("b", "b1").unwrap();
        graph_builder
            .assign(&b1, PropertyId::from("input-integer"), Value::Integer(42))
            .unwrap();
        let c1 = graph_builder.node("c", "c1").unwrap();

        graph_builder
            .connect(
                &a1,
                PropertyId::from("event"),
                &b1,
                PropertyId::from("command"),
            )
            .unwrap();
        graph_builder
            .connect(
                &b1,
                PropertyId::from("event"),
                &c1,
                PropertyId::from("command"),
            )
            .unwrap();
        graph_builder
            .connect(
                &b1,
                PropertyId::from("output-integer"),
                &c1,
                PropertyId::from("input-integer"),
            )
            .unwrap();

        graph_builder.build()
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
        let a1 = graph_builder.node("a", "a1").unwrap();
        let a1 = graph_builder.node("a", "a1").unwrap();
    })
    .is_err());

    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node("a", "a1").unwrap();
        graph_builder
            .assign(&a1, PropertyId::from("command"), Value::Integer(42))
            .unwrap();
    })
    .is_err());

    assert!(catch_unwind(|| {
        let schema = build_schema();
        let mut graph_builder = Graph::builder(&schema);
        let a1 = graph_builder.node("a", "a1").unwrap();
        let a1 = graph_builder.node("a", "a1").unwrap();
        graph_builder
            .connect(
                &a1,
                PropertyId::from("event"),
                &a1,
                PropertyId::from("command"),
            )
            .unwrap();
    })
    .is_err());
}

fn build_schema() -> Schema {
    Schema::builder()
        .node(
            Node::builder("a")
                .property(Property::Event {
                    id: PropertyId::from("event"),
                })
                .property(Property::Command {
                    id: PropertyId::from("command"),
                })
                .property(Property::Input {
                    id: PropertyId::from("input-string"),
                    data_type: DataType::String,
                    default_value: None,
                })
                .property(Property::Output {
                    id: PropertyId::from("output-string"),
                    data_type: DataType::String,
                    default_value: None,
                })
                .build(),
        )
        .node(
            Node::builder("b")
                .property(Property::Event {
                    id: PropertyId::from("event"),
                })
                .property(Property::Command {
                    id: PropertyId::from("command"),
                })
                .property(Property::Input {
                    id: PropertyId::from("input-integer"),
                    data_type: DataType::Integer,
                    default_value: None,
                })
                .property(Property::Output {
                    id: PropertyId::from("output-integer"),
                    data_type: DataType::Integer,
                    default_value: None,
                })
                .build(),
        )
        .node(
            Node::builder("c")
                .property(Property::Event {
                    id: PropertyId::from("event"),
                })
                .property(Property::Command {
                    id: PropertyId::from("command"),
                })
                .property(Property::Input {
                    id: PropertyId::from("input-integer"),
                    data_type: DataType::Integer,
                    default_value: None,
                })
                .property(Property::Output {
                    id: PropertyId::from("output-integer"),
                    data_type: DataType::Integer,
                    default_value: None,
                })
                .build(),
        )
        .build()
}
