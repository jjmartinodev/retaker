use std::hash::Hash;

use hashbrown::HashMap;

pub struct SystemGroup<T> {
    systems: Vec<fn(&mut T) -> ()>
}

pub struct Scheduler<G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy, T> {
    systems: HashMap<G, SystemGroup<T>>
}

impl<G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy, T> Scheduler<G, T> {
    pub fn new() -> Scheduler<G, T> {
        Scheduler { systems: HashMap::new() }
    }
    pub fn add_system(&mut self, system: fn(&mut T) -> (), group: G) {
        if let Some(group) = self.systems.get_mut(&group) {
            group.systems.push(system);
        } else {
            self.systems.insert(group, SystemGroup {
                systems: vec![system]
            });
        }
    }
    pub fn add_systems<const N: usize>(&mut self, systems: [fn(&mut T) -> (); N], group: G) {
        if let Some(group) = self.systems.get_mut(&group) {
            group.systems.extend_from_slice(&systems);
        } else {
            self.systems.insert(group, SystemGroup {
                systems: systems.to_vec()
            });
        }
    }
    pub fn run_group(&self, state: &mut T, group: G) {
        if let Some(group) = self.systems.get(&group) {
            for system in &group.systems {
                system(state);
            }
        } else {
            panic!("tried to run an unregistered group")
        }
    }
}

impl<G: Hash + Eq + PartialEq + PartialOrd + Ord + Clone + Copy, T> Default for Scheduler<G, T> {
    fn default() -> Self {
        Self::new()
    }
}