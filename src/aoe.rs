use std::any::Any;

use hashbrown::HashMap;
use varset::VarSet;

use crate::entity::EntityId;

/// ## Array Of Entities
///
/// Stores entity components in containers separated by entity
pub struct AOEStorage {
    entities: HashMap<EntityId, VarSet>,
}

impl AOEStorage {
    pub fn new() -> AOEStorage {
        AOEStorage {
            entities: HashMap::new(),
        }
    }
    pub fn raw_map(&self) -> &HashMap<EntityId, VarSet> {
        &self.entities
    }
    pub fn insert_entity(&mut self, entity: &EntityId) {
        if self.entities.contains_key(entity) {
            unreachable!("this should never happen")
        }
        self.entities.insert(*entity, VarSet::new());
    }
    pub fn remove_entity(&mut self, entity: &EntityId) {
        self.entities.remove(entity);
    }
    pub fn insert_component<C: Any>(&mut self, entity: &EntityId, component: C) {
        self.entities
            .get_mut(entity)
            .unwrap()
            .insert::<C>(component);
    }
    pub fn remove_component<C: Any>(&mut self, entity: &EntityId) -> Option<C> {
        self.entities.get_mut(entity).unwrap().remove::<C>()
    }
    pub fn get_component<C: Any>(&self, entity: &EntityId) -> Option<&C> {
        self.entities.get(entity).unwrap().get::<C>()
    }
    pub fn get_mut_component<C: Any>(&mut self, entity: &EntityId) -> Option<&mut C> {
        self.entities.get_mut(entity).unwrap().get_mut::<C>()
    }
    pub fn has_component<C: Any>(&self, entity: &EntityId) -> bool {
        self.entities
            .get(entity)
            .expect("entity doesn't exist")
            .get::<C>()
            .is_some()
    }
}

impl Default for AOEStorage {
    fn default() -> Self {
        Self::new()
    }
}