//! Contains schema utils.

use graph::schema::node::Node;
use graph::schema::property::{
    CommandProperty, EventProperty, InputProperty, OutputProperty, Property,
};
use graph::schema::Schema;

use crate::error::EngineError;

pub mod basic;

pub struct Library {
    pub schema: Schema,
}

pub struct CommandReference {
    pub node: Node,
    pub property: CommandProperty,
}

pub struct EventReference {
    pub node: Node,
    pub property: EventProperty,
}

pub struct InputReference {
    pub node: Node,
    pub property: InputProperty,
}

pub struct OutputReference {
    pub node: Node,
    pub property: OutputProperty,
}

impl Library {
    /// Returns a `Schema` supported by current implementation of the engine.
    pub fn get() -> Self {
        let schema = Schema::builder()
            .node(basic::action::get())
            .node(basic::integer::get())
            .node(basic::minus::get())
            .node(basic::plus::get())
            .node(basic::printer::get())
            .node(basic::repeat::get())
            .build();
        Library { schema }
    }

    pub fn get_command(
        &self,
        node_id: &str,
        command_id: &str,
    ) -> Result<CommandReference, EngineError> {
        let node = self.schema.nodes.get(node_id).unwrap();
        let property = node.properties.get(command_id).unwrap();
        if let Property::Command(property) = property {
            Ok(CommandReference {
                node: node.clone(),
                property: property.clone(),
            })
        } else {
            Err(EngineError::from(format!(
                "Property '{}' is not a command.",
                command_id
            )))
        }
    }

    pub fn get_event(&self, node_id: &str, event_id: &str) -> Result<EventReference, EngineError> {
        let node = self.schema.nodes.get(node_id).unwrap();
        let property = node.properties.get(event_id).unwrap();
        if let Property::Event(property) = property {
            Ok(EventReference {
                node: node.clone(),
                property: property.clone(),
            })
        } else {
            Err(EngineError::from(format!(
                "Property '{}' is not an event.",
                event_id
            )))
        }
    }

    pub fn get_input(&self, node_id: &str, input_id: &str) -> Result<InputReference, EngineError> {
        let node = self.schema.nodes.get(node_id).unwrap();
        let property = node.properties.get(input_id).unwrap();
        if let Property::Input(property) = property {
            Ok(InputReference {
                node: node.clone(),
                property: property.clone(),
            })
        } else {
            Err(EngineError::from(format!(
                "Property '{}' is not an input.",
                input_id
            )))
        }
    }

    pub fn get_output(
        &self,
        node_id: &str,
        output_id: &str,
    ) -> Result<OutputReference, EngineError> {
        let node = self.schema.nodes.get(node_id).unwrap();
        let property = node.properties.get(output_id).unwrap();
        if let Property::Output(property) = property {
            Ok(OutputReference {
                node: node.clone(),
                property: property.clone(),
            })
        } else {
            Err(EngineError::from(format!(
                "Property '{}' is not an output.",
                output_id
            )))
        }
    }
}
