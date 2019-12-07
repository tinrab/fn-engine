use std::sync::Weak;

use crossbeam::channel::{Receiver, Sender};

use crate::library::Library;
use crate::message::{Instruction, Message};
use crate::processor::action_processor::ActionProcessor;
use crate::processor::Processor;

pub struct Worker {
    id: u64,
    outbox: Sender<Message>,
    inbox: Receiver<Message>,
    library: Weak<Library>,
    processors: Vec<Box<dyn Processor>>,
}

impl Worker {
    pub fn new(
        id: u64,
        outbox: Sender<Message>,
        inbox: Receiver<Message>,
        library: Weak<Library>,
    ) -> Self {
        Worker {
            id,
            outbox,
            inbox,
            library,
            processors: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        self.register_processors();

        for message in self.inbox.iter() {
            match message {
                Message::Instruction(instruction) => self.handle_instruction(instruction),
            }
        }
    }

    fn register_processors(&mut self) {
        let library = self.library.upgrade().unwrap();
        self.processors
            .push(Box::new(ActionProcessor::new(&library)));
    }

    fn handle_instruction(&self, instruction: Instruction) {
        println!("[{}] Handling {:?}...", self.id, instruction.command_id);
    }
}
