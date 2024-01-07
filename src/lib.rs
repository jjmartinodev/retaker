use state::State;
use system::System;

mod entity;
mod component;
mod system;
mod state;

#[cfg(test)]
mod tests;

pub struct App {
    systems:Vec<System>,
    start_system: System
}

impl App {
    pub fn new(start_system: System) -> App {
        App { systems: vec![], start_system }
    }
    pub fn add_system(mut self, system: System) -> Self {
        self.systems.push(system);
        self
    }
    pub fn uptade(&self, state: &mut State) {
        for system in self.systems.iter() {
            (system)(state)
        }
    }
    pub fn run(self) {
        let mut state = State::new();

        (self.start_system)(&mut state);

        while !state.exiting() {
            self.uptade(&mut state);
        }
    }
}