pub trait State {
    fn alive(&self) -> bool {false}
}

pub enum System<T> {
    Start (fn(&mut T) -> ()),
    Uptade (fn(&mut T) -> ()),
    Exit (fn(&mut T) -> ()),
}

pub struct Scheduler<T: State> {
    state: T,
    systems: Vec<System<T>>,
}

impl<T: State> Scheduler<T> {
    pub fn new(state: T) -> Scheduler<T> {
        Scheduler {
            state,
            systems: vec![],
        }
    }
    pub fn add_systems<const N: usize>(&mut self, systems: [System<T>; N]) {
        for system in systems {
            self.systems.push(system);
        }
    }
    pub fn run(&mut self) {
        for system in &self.systems {
            match system {
                System::Start(system) => system(&mut self.state),
                _ => ()
            }
        }
        'main: loop {
            if self.systems.len() == 0 {
                break 'main;
            }
            for system in &self.systems {
                match system {
                    System::Uptade(system) => system(&mut self.state),
                    _ => ()
                }
                if !self.state.alive() {
                    break 'main;
                } else {
                    continue;
                }
            }
        }
        for system in &self.systems {
            match system {
                System::Exit(system) => system(&mut self.state),
                _ => ()
            }
        }
    }
}