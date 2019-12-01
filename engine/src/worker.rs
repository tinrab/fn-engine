use crossbeam::channel::{Receiver, Sender};

use crate::message::{Instruction, Message};

pub struct Worker {
    id: u64,
    outbox: Sender<Message>,
    inbox: Receiver<Message>,
}

impl Worker {
    pub fn new(id: u64, outbox: Sender<Message>, inbox: Receiver<Message>) -> Self {
        Worker { id, outbox, inbox }
    }

    pub fn run(&self) {
        for message in self.inbox.iter() {
            match message {
                Message::Instruction(instruction) => self.handle_instruction(instruction),
            }
        }
    }

    fn handle_instruction(&self, instruction: Instruction) {
        println!("[{}] Handling {:?}...", self.id, instruction.command_id);
    }
}
