use engine::{Engine, EngineConfig};
use graph::error::GraphError;
use graph::graph::Graph;
use graph::schema::Schema;
use graph::value::Value;

#[test]
fn basic() {
    let schema = engine::schema::get();
    let build_graph = |schema: &Schema| -> Result<Graph, GraphError> {
        let mut gb = Graph::builder(schema);
        let a1 = gb.node("action", "a1")?;
        let r1 = gb.node("repeat", "r1")?;
        let p1 = gb.node("printer", "p1")?;
        let minus = gb.node("minus", "minus")?;
        let plus = gb.node("plus", "plus")?;
        let six = gb.node("integer", "six")?;
        let four = gb.node("integer", "four")?;
        let three = gb.node("integer", "three")?;

        gb.assign(&r1, "times", Value::Integer(3))?;
        gb.assign(&six, "value", Value::Integer(6))?;
        gb.assign(&four, "value", Value::Integer(4))?;
        gb.assign(&three, "value", Value::Integer(3))?;

        gb.connect(&a1, "triggered", &r1, "start")?;
        gb.connect(&r1, "executed", &p1, "print")?;
        gb.connect(&six, "return-value", &plus, "a")?;
        gb.connect(&four, "return-value", &plus, "b")?;
        gb.connect(&plus, "c", &minus, "a")?;
        gb.connect(&three, "return-value", &minus, "b")?;
        gb.connect(&minus, "c", &p1, "content")?;

        gb.build()
    };

    let graph = build_graph(&schema).unwrap();
    let engine_config = EngineConfig::load().unwrap();
    let mut engine = Engine::new(engine_config, schema);

    engine.run();
    engine.execute(graph);
}
