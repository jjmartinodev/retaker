use crate::{system::System, world::World};

pub struct Scheduler {
    start_systems: Vec<System>,
    uptade_systems: Vec<System>,
    exit_systems: Vec<System>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            start_systems: vec![],
            uptade_systems: vec![],
            exit_systems: vec![],
        }
    }
    pub fn add_system(&mut self, system: System) {
        match system {
            System::Start(_) => self.start_systems.push(system),
            System::Uptade(_) => self.uptade_systems.push(system),
            System::Exit(_) => self.exit_systems.push(system),
        }
    }
    pub fn start(&mut self, world: &mut World) {
        for system in &self.start_systems {
            if let System::Start(start) = system {
                start(world);
            }
        }
    }
    pub fn uptade(&mut self, world: &mut World) {
        for system in &self.uptade_systems {
            if let System::Uptade(uptade) = system {
                uptade(world);
            }
        }
    }
    pub fn exit(&mut self, world: &mut World) {
        for system in &self.exit_systems {
            if let System::Exit(exit) = system {
                exit(world);
            }
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}