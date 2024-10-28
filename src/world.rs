use std::any::{Any, TypeId};

use hashbrown::HashMap;

use crate::aoe::AOEStorage;
use crate::entity::{EntityId, EntityMut, EntityRef};

use crate::aoc::AOCStorage;

pub enum EntityStorage {
    AOC(AOCStorage),
    AOE(AOEStorage),
}

pub struct World {
    storage: EntityStorage,
    next_entity_id: u32,
    unique_entities: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new(storage: EntityStorage) -> World {
        World {
            storage,
            next_entity_id: 0,
            unique_entities: HashMap::new(),
        }
    }
    pub fn ref_entity<'a>(&'a self, entity: &EntityId) -> EntityRef<'a> {
        EntityRef {
            id: *entity,
            storage: &self.storage
        }
    }
    pub fn mut_entity<'a>(&'a mut self, entity: &EntityId) -> EntityMut<'a> {
        EntityMut {
            id: *entity,
            storage: &mut self.storage
        }
    }
    pub fn query<With: Any + 'static>(&self) -> Vec<EntityId> {
        match &self.storage {
            EntityStorage::AOC(aoc) => aoc
                .get_component_list::<With>()
                .unwrap()
                .keys()
                .copied()
                .collect(),
            EntityStorage::AOE(aoe) => {
                let map = aoe.raw_map();
                let mut entities = vec![];
                for (entity, component_list) in map {
                    if component_list.contains::<With>() {
                        entities.push(*entity);
                    }
                }
                entities
            }
        }
    }
    pub fn insert_unique_entity<State: Any>(&mut self, state: State) {
        assert!(
            !self.unique_entities.contains_key(&TypeId::of::<State>()),
            "tried to insert a unique entity when it already was registered!"
        );
        self.unique_entities
            .insert(TypeId::of::<State>(), Box::new(state));
    }
    pub fn remove_unique_entity<State: Any>(&mut self, state: State) {
        assert!(
            self.unique_entities.contains_key(&TypeId::of::<State>()),
            "tried to remove a unique entity when it wasn't registered!"
        );
        self.unique_entities
            .insert(TypeId::of::<State>(), Box::new(state));
    }
    pub fn get_unique_entity<State: Any>(&self) -> Option<&State> {
        self.unique_entities
            .get(&TypeId::of::<State>())
            .expect("didn't find unique entity")
            .downcast_ref()
    }
    pub fn get_mut_unique_entity<State: Any>(&mut self) -> Option<&mut State> {
        self.unique_entities
            .get_mut(&TypeId::of::<State>())
            .expect("didn't find unique entity")
            .downcast_mut()
    }
    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        match &mut self.storage {
            EntityStorage::AOE(aoe) => aoe.insert_entity(&id),
            _ => (),
        }
        id
    }
    pub fn delete_entity(&mut self, entity: &EntityId) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => aoc.remove_entity(entity),
            EntityStorage::AOE(aoe) => aoe.remove_entity(entity),
        }
    }
    pub fn get_component<C: Any + 'static>(&self, entity: &EntityId) -> Option<&C> {
        match &self.storage {
            EntityStorage::AOC(aoc) => aoc.get_component::<C>(entity),
            EntityStorage::AOE(aoe) => aoe.get_component::<C>(entity),
        }
    }
    pub fn get_mut_component<C: Any + 'static>(&mut self, entity: &EntityId) -> Option<&mut C> {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => aoc.get_mut_component::<C>(entity),
            EntityStorage::AOE(aoe) => aoe.get_mut_component::<C>(entity),
        }
    }
    pub fn insert_component<C: Any + 'static>(&mut self, entity: &EntityId, component: C) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => aoc.insert_component::<C>(entity, component),
            EntityStorage::AOE(aoe) => aoe.insert_component::<C>(entity, component),
        }
    }
    pub fn remove_component<C: Any + 'static>(&mut self, entity: &EntityId) -> Option<C> {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => aoc.remove_component::<C>(entity),
            EntityStorage::AOE(aoe) => aoe.remove_component::<C>(entity),
        }
    }
}
