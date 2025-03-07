pub type System<W> = fn(&W) -> ();

pub struct Work<W> {
    systems: Vec<System<W>>
}

pub struct ThreadedWork<W> {
    systems: Vec<System<W>>
}

impl<W> Work<W> {
    pub fn new() -> Work<W> {
        Work {
            systems: vec![]
        }
    }
    pub fn add_system(mut self, system: System<W>) -> Self {
        self.systems.push(system);
        self
    }
    pub fn run(&self, world: &W) {
        for system in &self.systems {
            system(world)
        }
    }
}


impl<W: Sync> ThreadedWork<W> {
    pub fn new() -> ThreadedWork<W> {
        ThreadedWork {
            systems: vec![]
        }
    }
    pub fn add_system(mut self, system: System<W>) -> Self {
        self.systems.push(system);
        self
    }
    pub fn run(&self, world: &W) {
        for system in &self.systems {
            rayon::scope(|scope| {
                scope.spawn(|_| {
                    system(world)
                });
            });
        }
    }
}