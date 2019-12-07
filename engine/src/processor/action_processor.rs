use std::sync::Arc;

use crate::library;
use crate::library::Library;
use crate::processor::{Processor, Router};

pub struct ActionProcessor {
    router: Router,
}

impl ActionProcessor {
    pub fn new(lib: &Arc<Library>) -> Self {
        let trigger_command = lib.get_command(
            library::basic::action::ID,
            library::basic::action::COMMAND_TRIGGER,
        );
        let triggered_event = lib.get_event(
            library::basic::action::ID,
            library::basic::action::EVENT_TRIGGERED,
        );

        let router = Router::new();
        ActionProcessor { router }
    }

    fn handle_trigger(&self) {}
}

impl Processor for ActionProcessor {
    fn router(&self) -> &Router {
        &self.router
    }
}
