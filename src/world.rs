use std::any::{Any, TypeId};

use hashbrown::HashMap;
use parking_lot::RwLock;

use crate::{
    component::{ComponentMut, ComponentReadGuard, ComponentRef, ComponentWriteGuard, ManyComponentMut},
    entity::EntityId,
};

pub trait ComponentList {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn untyped_remove(&mut self, entity: &EntityId);
}

impl<T: Any> ComponentList for HashMap<EntityId, T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    fn untyped_remove(&mut self, entity: &EntityId) {
        self.remove(entity);
    }
}

pub struct World {
    component_lists: HashMap<TypeId, RwLock<Box<dyn ComponentList>>>,
}

#[derive(Debug, Clone)]
pub struct Query {
    entities: Vec<EntityId>,
}

impl World {
    pub fn new() -> World {
        World {
            component_lists: HashMap::new(),
        }
    }
    pub fn has_component<C: Any>(&self, entity: &EntityId) -> bool {
        self.component_lists
            .get(&TypeId::of::<C>())
            .unwrap()
            .read()
            .as_any()
            .downcast_ref::<HashMap<EntityId, C>>()
            .unwrap()
            .contains_key(entity)
    }
    pub fn remove_entity(&self, entity: EntityId) {
        for list in self.component_lists.values() {
            list.write().untyped_remove(&entity);
        }
    }
    fn register_component<C: Any>(&mut self) {
        if self.component_lists.contains_key(&TypeId::of::<C>()) {
            unreachable!("tried to register a component type once again")
        } else {
            self.component_lists.insert(
                TypeId::of::<C>(),
                RwLock::new(Box::new(HashMap::<EntityId, C>::new())),
            );
        }
    }
    pub fn remove_component<C: Any>(&self, entity: &EntityId) -> Option<C> {
        if let Some(list) = self.component_lists.get(&TypeId::of::<C>()) {
            list.write()
                .as_mut_any()
                .downcast_mut::<HashMap<EntityId, C>>()
                .unwrap()
                .remove(entity)
        } else {
            None
        }
    }
    pub fn mut_component_list<'a, C: Any>(&'a self) -> Option<ComponentWriteGuard<'a>> {
        if let Some(list) = self.component_lists.get(&TypeId::of::<C>()) {
            if list.is_locked() {
                return None;
            }
            Some(ComponentWriteGuard::new::<C>(list))
        } else {
            None
        }
    }
    pub fn component_list<'a, C: Any>(&'a self) -> Option<ComponentReadGuard<'a>> {
        if let Some(list) = self.component_lists.get(&TypeId::of::<C>()) {
            if list.is_locked_exclusive() {
                return None;
            }
            Some(ComponentReadGuard::new::<C>(list))
        } else {
            None
        }
    }
    pub fn component<'a, C: Any>(&'a self, entity: &EntityId) -> Option<ComponentRef<'a, C>> {
        if let Some(list) = self.component_list::<C>() {
            Some(ComponentRef::new(list, entity))
        } else {
            None
        }
    }
    pub fn mut_component<'a, C: Any>(&'a self, entity: &EntityId) -> Option<ComponentMut<'a, C>> {
        if let Some(list) = self.mut_component_list::<C>() {
            Some(ComponentMut::new(list, entity))
        } else {
            None
        }
    }
    pub fn many_mut_component<'a, C: Any, const N: usize>(
        &'a self,
        entities: [EntityId; N],
    ) -> Option<ManyComponentMut<'a, C, N>> {
        if let Some(list) = self.mut_component_list::<C>() {
            Some(ManyComponentMut::new(list, entities))
        } else {
            None
        }
    }
    pub fn query<C: Any>(&self) -> Query {
        if let Some(list) = self.component_lists.get(&TypeId::of::<C>()) {
            Query {
                entities: list
                    .read()
                    .as_any()
                    .downcast_ref::<HashMap<EntityId, C>>()
                    .unwrap()
                    .keys()
                    .cloned()
                    .collect(),
            }
        } else {
            panic!("tried to query an unregistered component")
        }
    }
    pub fn insert_component<C: Any>(&mut self, entity: &EntityId, component: C) {
        let list = self.component_lists.get_mut(&TypeId::of::<C>());
        if let Some(list) = list {
            list.write()
                .as_mut_any()
                .downcast_mut::<HashMap<EntityId, C>>()
                .unwrap()
                .insert(*entity, component);
        } else {
            self.register_component::<C>();
            self.component_lists
                .get_mut(&TypeId::of::<C>())
                .unwrap()
                .write()
                .as_mut_any()
                .downcast_mut::<HashMap<EntityId, C>>()
                .unwrap()
                .insert(*entity, component);
        }
    }
    pub fn insert_component2<C1: Any, C2: Any>(&mut self, entity: &EntityId, components: (C1, C2)) {
        self.insert_component(entity, components.0);
        self.insert_component(entity, components.1);
    }
    pub fn insert_component3<C1: Any, C2: Any, C3: Any>(
        &mut self,
        entity: &EntityId,
        components: (C1, C2, C3),
    ) {
        self.insert_component(entity, components.0);
        self.insert_component(entity, components.1);
        self.insert_component(entity, components.2);
    }
    pub fn insert_component4<C1: Any, C2: Any, C3: Any, C4: Any>(
        &mut self,
        entity: &EntityId,
        components: (C1, C2, C3, C4),
    ) {
        self.insert_component(entity, components.0);
        self.insert_component(entity, components.1);
        self.insert_component(entity, components.2);
        self.insert_component(entity, components.3);
    }
    pub fn insert_component5<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any>(
        &mut self,
        entity: &EntityId,
        components: (C1, C2, C3, C4, C5),
    ) {
        self.insert_component(entity, components.0);
        self.insert_component(entity, components.1);
        self.insert_component(entity, components.2);
        self.insert_component(entity, components.3);
        self.insert_component(entity, components.4);
    }
    pub fn insert_component6<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any, C6: Any>(
        &mut self,
        entity: &EntityId,
        components: (C1, C2, C3, C4, C5, C6),
    ) {
        self.insert_component(entity, components.0);
        self.insert_component(entity, components.1);
        self.insert_component(entity, components.2);
        self.insert_component(entity, components.3);
        self.insert_component(entity, components.4);
        self.insert_component(entity, components.5);
    }
    pub fn insert_component7<C1: Any, C2: Any, C3: Any, C4: Any, C5: Any, C6: Any, C7: Any>(
        &mut self,
        entity: &EntityId,
        components: (C1, C2, C3, C4, C5, C6, C7),
    ) {
        self.insert_component(entity, components.0);
        self.insert_component(entity, components.1);
        self.insert_component(entity, components.2);
        self.insert_component(entity, components.3);
        self.insert_component(entity, components.4);
        self.insert_component(entity, components.5);
        self.insert_component(entity, components.6);
    }
    pub fn insert_component8<
        C1: Any,
        C2: Any,
        C3: Any,
        C4: Any,
        C5: Any,
        C6: Any,
        C7: Any,
        C8: Any,
    >(
        &mut self,
        entity: &EntityId,
        components: (C1, C2, C3, C4, C5, C6, C7, C8),
    ) {
        self.insert_component(entity, components.0);
        self.insert_component(entity, components.1);
        self.insert_component(entity, components.2);
        self.insert_component(entity, components.3);
        self.insert_component(entity, components.4);
        self.insert_component(entity, components.5);
        self.insert_component(entity, components.6);
        self.insert_component(entity, components.7);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl Query {
    pub fn filter_with<C: Any>(&mut self, world: &mut World) {
        self.entities.retain(|e| world.has_component::<C>(e));
    }
    pub fn filter_without<C: Any>(&mut self, world: &mut World) {
        self.entities.retain(|e| !world.has_component::<C>(e));
    }
    pub fn filter_or<A: Any, B: Any>(&mut self, world: &mut World) {
        self.entities
            .retain(|e| world.has_component::<A>(e) || world.has_component::<B>(e));
    }
    pub fn filter_and<A: Any, B: Any>(&mut self, world: &mut World) {
        self.entities
            .retain(|e| world.has_component::<A>(e) && world.has_component::<B>(e));
    }
    pub fn filter_xor<A: Any, B: Any>(&mut self, world: &mut World) {
        self.entities
            .retain(|e| world.has_component::<A>(e) ^ world.has_component::<B>(e));
    }
    pub fn filter_nor<A: Any, B: Any>(&mut self, world: &mut World) {
        self.entities
            .retain(|e| !(world.has_component::<A>(e) || world.has_component::<B>(e)));
    }
    pub fn filter_nand<A: Any, B: Any>(&mut self, world: &mut World) {
        self.entities
            .retain(|e| !(world.has_component::<A>(e) && world.has_component::<B>(e)));
    }
    pub fn filter_xnor<A: Any, B: Any>(&mut self, world: &mut World) {
        self.entities
            .retain(|e| !(world.has_component::<A>(e) ^ world.has_component::<B>(e)));
    }
}

impl Iterator for Query {
    type Item = EntityId;
    fn next(&mut self) -> Option<Self::Item> {
        self.entities.pop()
    }
}
