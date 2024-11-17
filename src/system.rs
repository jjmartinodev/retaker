use std::hash::Hash;

use hashbrown::HashMap;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub struct ThreadedSystemGroup<T: Sync + Send> {
    systems: Vec<fn(&T) -> ()>,
}

pub struct SystemGroup<T> {
    systems: Vec<fn(&mut T) -> ()>,
}

unsafe impl<T: Sync + Send> Send for ThreadedSystemGroup<T> {}
unsafe impl<T: Sync + Send> Sync for ThreadedSystemGroup<T> {}

pub struct PooledScheduler<
    G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy,
    T: Sync + Send,
> {
    systems: HashMap<G, ThreadedSystemGroup<T>>,
    pool: ThreadPool,
}

pub struct Scheduler<G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy, T> {
    systems: HashMap<G, SystemGroup<T>>,
}

impl<G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy, T: Sync + Send>
    PooledScheduler<G, T>
{
    pub fn new(thread_count: Option<usize>) -> PooledScheduler<G, T> {
        PooledScheduler {
            systems: HashMap::new(),
            pool: ThreadPoolBuilder::new()
                .num_threads(thread_count.unwrap_or(0))
                .build()
                .unwrap(),
        }
    }
    pub fn add_system(&mut self, system: fn(&T) -> (), group: G) {
        if let Some(group) = self.systems.get_mut(&group) {
            group.systems.push(system);
        } else {
            self.systems.insert(
                group,
                ThreadedSystemGroup {
                    systems: vec![system],
                },
            );
        }
    }
    pub fn add_systems<const N: usize>(&mut self, systems: [fn(&T) -> (); N], group: G) {
        if let Some(group) = self.systems.get_mut(&group) {
            group.systems.extend_from_slice(&systems);
        } else {
            self.systems.insert(
                group,
                ThreadedSystemGroup {
                    systems: systems.to_vec(),
                },
            );
        }
    }
    pub fn run_group(&self, state: &T, group: G) {
        if let Some(group) = self.systems.get(&group) {
            self.pool.scope(|a| {
                group
                    .systems
                    .iter()
                    .for_each(|system| a.spawn(|_| system(state)))
            });
        } else {
            panic!("tried to run an unregistered group")
        }
    }
}

impl<G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy, T>
    Scheduler<G, T>
{
    pub fn new() -> Scheduler<G, T> {
        Scheduler {
            systems: HashMap::new(),
        }
    }
    pub fn add_system(&mut self, system: fn(&mut T) -> (), group: G) {
        if let Some(group) = self.systems.get_mut(&group) {
            group.systems.push(system);
        } else {
            self.systems.insert(
                group,
                SystemGroup {
                    systems: vec![system],
                },
            );
        }
    }
    pub fn add_systems<const N: usize>(&mut self, systems: [fn(&mut T) -> (); N], group: G) {
        if let Some(group) = self.systems.get_mut(&group) {
            group.systems.extend_from_slice(&systems);
        } else {
            self.systems.insert(
                group,
                SystemGroup {
                    systems: systems.to_vec(),
                },
            );
        }
    }
    pub fn run_group(&self, state: &mut T, group: G) {
        if let Some(group) = self.systems.get(&group) {
            group.systems.iter().for_each(|system| system(state));
        } else {
            panic!("tried to run an unregistered group")
        }
    }
}

impl<G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy, T: Send + Sync> Default
    for PooledScheduler<G, T>
{
    fn default() -> Self {
        Self::new(None)
    }
}
