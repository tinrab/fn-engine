#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unsafe_code,
    unreachable_pub
)]

//! Engine for executing graph based programs.

extern crate config;
extern crate crossbeam;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{env, thread};

use config::{Config, File};
use crossbeam::channel::{Receiver, Sender};
use crossbeam::sync::WaitGroup;

use graph::graph::Graph;
use graph::schema::Schema;

use crate::error::EngineError;
use crate::message::{Context, Message};
use crate::worker::Worker;

pub mod error;
pub mod message;
pub mod processor;
pub mod schema;
pub mod worker;

#[derive(Debug, Deserialize)]
pub struct WorkerConfig {
    pool_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    pub worker: WorkerConfig,
}

static DEFAULT_CONFIG_PATH: &str = "config.yaml";

impl EngineConfig {
    pub fn load() -> Result<EngineConfig, EngineError> {
        let mut config = Config::new();
        config.merge(File::with_name(DEFAULT_CONFIG_PATH))?;

        let path = env::var("APP_CONFIG_PATH");
        if let Ok(path) = path {
            config.merge(File::with_name(path.as_str()))?;
        }

        let engine_config: EngineConfig = config.try_into()?;

        Ok(engine_config)
    }
}

/// Used to coordinate executing of graphs.
pub struct Engine {
    /// Config for this engine.
    pub config: EngineConfig,
    /// Schema used by this engine.
    pub schema: Schema,

    message_sender: Sender<Message>,
    message_receiver: Receiver<Message>,
}

impl Engine {
    /// Constructs a new engine for executing graphs based on `schema`.
    pub fn new(config: EngineConfig, schema: Schema) -> Self {
        let (s, r) = crossbeam::channel::bounded::<Message>(10);
        Engine {
            config,
            schema,
            message_sender: s,
            message_receiver: r,
        }
    }

    /// Run the engine.
    pub fn run(&mut self) {
        for i in 0..self.config.worker.pool_size {
            let id = i as u64;
            let outbox = self.message_sender.clone();
            let inbox = self.message_receiver.clone();

            thread::spawn(move || {
                let worker = Worker::new(id, outbox, inbox);
                worker.run();
            });
        }
    }

    /// Execute a graph on this engine. An engine must be ran first.
    pub fn execute(&self, graph: Graph) {
        let graph = Arc::new(graph);
        let s2 = self.message_sender.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            let context = Context::new(Arc::clone(&graph), graph.get_node("a1"));
            s2.send(Message::instruction(context, "trigger"));
        });

        thread::sleep(Duration::from_secs(10));
    }
}
