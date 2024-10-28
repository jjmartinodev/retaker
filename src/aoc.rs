use std::any::{Any, TypeId};

use hashbrown::HashMap;

use crate::entity::EntityId;

pub trait AnyStorage: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn untyped_remove(&mut self, entity: &EntityId);
}

impl<T: Any + 'static> AnyStorage for HashMap<EntityId, T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn untyped_remove(&mut self, entity: &EntityId) {
        self.remove(entity);
    }
}

/// ## Array Of Component Storage
///
/// Stores entity components in containers separated by type
pub struct AOCStorage {
    components: HashMap<TypeId, Box<dyn AnyStorage>>,
}

impl AOCStorage {
    pub fn new() -> AOCStorage {
        AOCStorage {
            components: HashMap::new(),
        }
    }
    pub fn get_component_lists(&self) -> &HashMap<TypeId, Box<dyn AnyStorage>> {
        &self.components
    }
    pub fn get_mut_component_lists(&mut self) -> &mut HashMap<TypeId, Box<dyn AnyStorage>> {
        &mut self.components
    }
    pub fn remove_entity(&mut self, entity: &EntityId) {
        for component_list in &mut self.components {
            component_list.1.untyped_remove(entity);
        }
    }
    pub fn get_component_list<C: Any>(&self) -> Option<&HashMap<EntityId, C>> {
        if let Some(comp) = self.components.get(&TypeId::of::<C>()) {
            Some(
                comp.as_any()
                    .downcast_ref::<HashMap<EntityId, C>>()
                    .unwrap(),
            )
        } else {
            None
        }
    }
    pub fn get_mut_component_list<C: Any>(
        &mut self,
    ) -> Option<&mut HashMap<EntityId, C>> {
        if let Some(comp) = self.components.get_mut(&TypeId::of::<C>()) {
            Some(
                comp.as_any_mut()
                    .downcast_mut::<HashMap<EntityId, C>>()
                    .unwrap(),
            )
        } else {
            None
        }
    }
    pub fn insert_component<C: Any>(&mut self, entity: &EntityId, component: C) {
        if let Some(list) = self.get_mut_component_list::<C>() {
            if list.contains_key(entity) {
                panic!("tried to add a component to an entity that already has the same component")
            }
            list.insert(*entity, component);
        } else {
            self.components
                .insert(TypeId::of::<C>(), Box::new(HashMap::<EntityId, C>::new()));
            self.insert_component(entity, component);
        }
    }
    pub fn remove_component<C: Any>(&mut self, entity: &EntityId) -> Option<C> {
        if let Some(list) = self.get_mut_component_list::<C>() {
            list.remove(entity)
        } else {
            None
        }
    }
    pub fn get_component<C: Any>(&self, entity: &EntityId) -> Option<&C> {
        if let Some(list) = self.get_component_list::<C>() {
            list.get(entity)
        } else {
            None
        }
    }
    pub fn get_mut_component<C: Any>(&mut self, entity: &EntityId) -> Option<&mut C> {
        if let Some(list) = self.get_mut_component_list::<C>() {
            list.get_mut(entity)
        } else {
            None
        }
    }
}

impl Default for AOCStorage {
    fn default() -> Self {
        Self::new()
    }
}