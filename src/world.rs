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
            storage: &self.storage,
            entity: None,
        }
    }
    pub fn mut_entity<'a>(&'a mut self, entity: &EntityId) -> EntityMut<'a> {
        EntityMut {
            id: *entity,
            storage: &mut self.storage,
            entity: None,
        }
    }
    pub fn query<With: Any>(&self) -> Vec<EntityId> {
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
                    if component_list.components.contains::<With>() {
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
    pub fn unique_entity<State: Any>(&self) -> Option<&State> {
        self.unique_entities
            .get(&TypeId::of::<State>())
            .expect("didn't find unique entity")
            .downcast_ref()
    }
    pub fn mut_unique_entity<State: Any>(&mut self) -> Option<&mut State> {
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
    pub fn component<C: Any>(&self, entity: &EntityId) -> Option<&C> {
        match &self.storage {
            EntityStorage::AOC(aoc) => aoc.get_component::<C>(entity),
            EntityStorage::AOE(aoe) => aoe.component::<C>(entity),
        }
    }
    pub fn mut_component<C: Any>(&mut self, entity: &EntityId) -> Option<&mut C> {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => aoc.get_mut_component::<C>(entity),
            EntityStorage::AOE(aoe) => aoe.mut_component::<C>(entity),
        }
    }
    pub fn insert_component<C: Any>(&mut self, entity: &EntityId, component: C) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => aoc.insert_component::<C>(entity, component),
            EntityStorage::AOE(aoe) => aoe.insert_component::<C>(entity, component),
        }
    }
    pub fn insert_component2<C1: Any, C2: Any>(
        &mut self,
        entity: &EntityId,
        c1: C1,
        c2: C2,
    ) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.insert_component::<C1>(entity, c1);
                aoc.insert_component::<C2>(entity, c2);
            },
            EntityStorage::AOE(aoe) => {
                aoe.insert_component::<C1>(entity, c1);
                aoe.insert_component::<C2>(entity, c2);
            },
        }
    }
    pub fn insert_component3<C1: Any, C2: Any, C3: Any>(
        &mut self,
        entity: &EntityId,
        c1: C1,
        c2: C2,
        c3: C3,
    ) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.insert_component::<C1>(entity, c1);
                aoc.insert_component::<C2>(entity, c2);
                aoc.insert_component::<C3>(entity, c3);
            },
            EntityStorage::AOE(aoe) => {
                aoe.insert_component::<C1>(entity, c1);
                aoe.insert_component::<C2>(entity, c2);
                aoe.insert_component::<C3>(entity, c3);
            },
        }
    }
    pub fn insert_component4<C1: Any, C2: Any, C3: Any, C4: Any>(
        &mut self,
        entity: &EntityId,
        c1: C1,
        c2: C2,
        c3: C3,
        c4: C4,
    ) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.insert_component::<C1>(entity, c1);
                aoc.insert_component::<C2>(entity, c2);
                aoc.insert_component::<C3>(entity, c3);
                aoc.insert_component::<C4>(entity, c4);
            },
            EntityStorage::AOE(aoe) => {
                aoe.insert_component::<C1>(entity, c1);
                aoe.insert_component::<C2>(entity, c2);
                aoe.insert_component::<C3>(entity, c3);
                aoe.insert_component::<C4>(entity, c4);
            },
        }
    }
    pub fn insert_component5<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any>(
        &mut self,
        entity: &EntityId,
        c1: C1,
        c2: C2,
        c3: C3,
        c4: C4,
        c5: C5,
    ) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.insert_component::<C1>(entity, c1);
                aoc.insert_component::<C2>(entity, c2);
                aoc.insert_component::<C3>(entity, c3);
                aoc.insert_component::<C4>(entity, c4);
                aoc.insert_component::<C5>(entity, c5);
            },
            EntityStorage::AOE(aoe) => {
                aoe.insert_component::<C1>(entity, c1);
                aoe.insert_component::<C2>(entity, c2);
                aoe.insert_component::<C3>(entity, c3);
                aoe.insert_component::<C4>(entity, c4);
                aoe.insert_component::<C5>(entity, c5);
            },
        }
    }
    pub fn insert_component6<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any, C6: Any>(
        &mut self,
        entity: &EntityId,
        c1: C1,
        c2: C2,
        c3: C3,
        c4: C4,
        c5: C5,
        c6: C6,
    ) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.insert_component::<C1>(entity, c1);
                aoc.insert_component::<C2>(entity, c2);
                aoc.insert_component::<C3>(entity, c3);
                aoc.insert_component::<C4>(entity, c4);
                aoc.insert_component::<C5>(entity, c5);
                aoc.insert_component::<C6>(entity, c6);
            },
            EntityStorage::AOE(aoe) => {
                aoe.insert_component::<C1>(entity, c1);
                aoe.insert_component::<C2>(entity, c2);
                aoe.insert_component::<C3>(entity, c3);
                aoe.insert_component::<C4>(entity, c4);
                aoe.insert_component::<C5>(entity, c5);
                aoe.insert_component::<C6>(entity, c6);
            },
        }
    }
    pub fn insert_component7<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any, C6: Any, C7: Any>(
        &mut self,
        entity: &EntityId,
        c1: C1,
        c2: C2,
        c3: C3,
        c4: C4,
        c5: C5,
        c6: C6,
        c7: C7,
    ) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.insert_component::<C1>(entity, c1);
                aoc.insert_component::<C2>(entity, c2);
                aoc.insert_component::<C3>(entity, c3);
                aoc.insert_component::<C4>(entity, c4);
                aoc.insert_component::<C5>(entity, c5);
                aoc.insert_component::<C6>(entity, c6);
                aoc.insert_component::<C7>(entity, c7);
            },
            EntityStorage::AOE(aoe) => {
                aoe.insert_component::<C1>(entity, c1);
                aoe.insert_component::<C2>(entity, c2);
                aoe.insert_component::<C3>(entity, c3);
                aoe.insert_component::<C4>(entity, c4);
                aoe.insert_component::<C5>(entity, c5);
                aoe.insert_component::<C6>(entity, c6);
                aoe.insert_component::<C7>(entity, c7);
            },
        }
    }
    pub fn insert_component8<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any, C6: Any, C7: Any, C8: Any>(
        &mut self,
        entity: &EntityId,
        c1: C1,
        c2: C2,
        c3: C3,
        c4: C4,
        c5: C5,
        c6: C6,
        c7: C7,
        c8: C8,
    ) {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.insert_component::<C1>(entity, c1);
                aoc.insert_component::<C2>(entity, c2);
                aoc.insert_component::<C3>(entity, c3);
                aoc.insert_component::<C4>(entity, c4);
                aoc.insert_component::<C5>(entity, c5);
                aoc.insert_component::<C6>(entity, c6);
                aoc.insert_component::<C7>(entity, c7);
                aoc.insert_component::<C8>(entity, c8);
            },
            EntityStorage::AOE(aoe) => {
                aoe.insert_component::<C1>(entity, c1);
                aoe.insert_component::<C2>(entity, c2);
                aoe.insert_component::<C3>(entity, c3);
                aoe.insert_component::<C4>(entity, c4);
                aoe.insert_component::<C5>(entity, c5);
                aoe.insert_component::<C6>(entity, c6);
                aoe.insert_component::<C7>(entity, c7);
                aoe.insert_component::<C8>(entity, c8);
            },
        }
    }
    pub fn remove_component<C: Any>(&mut self, entity: &EntityId) -> Option<C> {
        match &mut self.storage {
            EntityStorage::AOC(aoc) => aoc.remove_component::<C>(entity),
            EntityStorage::AOE(aoe) => aoe.remove_component::<C>(entity),
        }
    }
}
