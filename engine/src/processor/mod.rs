pub mod action_processor;

pub struct Router {}

pub trait Processor {
    fn router(&self) -> &Router;
}

impl Router {
    pub fn new() -> Self {
        Router {}
    }
}
