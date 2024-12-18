use std::hash::Hash;

use hashbrown::HashMap;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub type System<State> = fn(&mut State) -> ();

pub struct Scheduler<State, Group: PartialEq + Eq + Hash> {
    systems: HashMap<Group, Vec<System<State>>>,
}

pub type ThreadedSystem<State> = fn(&State) -> ();

pub struct ThreadedScheduler<State: Send + Sync, Group: PartialEq + Eq + Hash> {
    systems: HashMap<Group, Vec<ThreadedSystem<State>>>,
    thread_pool: ThreadPool,
}

impl<State, Group: PartialEq + Eq + Hash> Scheduler<State, Group> {
    pub fn new() -> Scheduler<State, Group> {
        Scheduler {
            systems: HashMap::new(),
        }
    }
    pub fn add_systems<const N: usize>(
        mut self,
        systems: [System<State>; N],
        group: Group,
    ) -> Self {
        if let Some(appendable) = self.systems.get_mut(&group) {
            appendable.append(&mut systems.to_vec());
        } else {
            self.systems.insert(group, systems.to_vec());
        }
        self
    }
    pub fn run_group(&self, state: &mut State, group: &Group) {
        self.systems
            .get(group)
            .expect("system group not found")
            .iter()
            .for_each(|s| s(state));
    }
}

impl<State: Send + Sync, Group: PartialEq + Eq + Hash> ThreadedScheduler<State, Group> {
    pub fn new(num_threads: usize) -> ThreadedScheduler<State, Group> {
        ThreadedScheduler {
            systems: HashMap::new(),
            thread_pool: ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .unwrap(),
        }
    }
    pub fn add_systems<const N: usize>(
        mut self,
        systems: [ThreadedSystem<State>; N],
        group: Group,
    ) -> Self {
        if let Some(appendable) = self.systems.get_mut(&group) {
            appendable.append(&mut systems.to_vec());
        } else {
            self.systems.insert(group, systems.to_vec());
        }
        self
    }
    pub fn run_group_local(&self, state: &State, group: &Group) {
        self.systems
            .get(group)
            .expect("system group not found")
            .iter()
            .for_each(|s| s(state));
    }
    pub fn run_group_par(&self, state: &State, group: &Group) {
        if let Some(group) = self.systems.get(group) {
            self.thread_pool.scope(|scope| {
                for system in group {
                    scope.spawn(|_| system(state));
                }
            })
        } else {
            panic!("system group not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadedScheduler;

    #[test]
    fn test_multithreaded_scheduler() {
        #[derive(Hash, PartialEq, Eq)]
        enum Group {
            RunOnce,
        }

        let scheduler: ThreadedScheduler<(), Group> =
            ThreadedScheduler::new(2).add_systems([|_| {}, |_| {}], Group::RunOnce);

        scheduler.run_group_par(&(), &Group::RunOnce);
    }
}
