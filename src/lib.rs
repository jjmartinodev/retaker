use state::State;
use system::System;

mod tests;

mod state;
mod system;

pub struct App {
    start_fn: System,
    systems: Vec<System>
}

impl App {
    pub fn new(start_fn: System) -> App {
        App { start_fn, systems: vec![] }
    }
    pub fn add_system(mut self, system: System) -> App {
        self.systems.push(system);
        self
    }
    pub fn run(self) {

        let mut state = State::new();

        (self.start_fn)(&mut state);

        for system in &self.systems {
            system(&mut state);

            if state.exiting() {
                break;
            }
        }
    }
}