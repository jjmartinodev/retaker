pub enum System<T> {
    Start (fn(&mut T) -> ()),
    Uptade (fn(&mut T) -> ()),
    Exit (fn(&mut T) -> ()),
}

pub struct Scheduler<T> {
    start_systems: Vec<System<T>>,
    uptade_systems: Vec<System<T>>,
    exit_systems: Vec<System<T>>,
}

impl<T> Scheduler<T> {
    pub fn new() -> Scheduler<T> {
        Scheduler {
            start_systems: vec![],
            uptade_systems: vec![],
            exit_systems: vec![],
        }
    }
    pub fn add_system(&mut self, system: System<T>) {
        match system {
            System::Start(_) => self.start_systems.push(system),
            System::Uptade(_) => self.uptade_systems.push(system),
            System::Exit(_) => self.exit_systems.push(system),
        }
    }
    pub fn start(&mut self, handler: &mut T) {
        for system in &self.start_systems {
            if let System::Start(start) = system {
                start(handler);
            }
        }
    }
    pub fn uptade(&mut self, handler: &mut T) {
        for system in &self.uptade_systems {
            if let System::Uptade(uptade) = system {
                uptade(handler);
            }
        }
    }
    pub fn exit(&mut self, handler: &mut T) {
        for system in &self.exit_systems {
            if let System::Exit(exit) = system {
                exit(handler);
            }
        }
    }
}