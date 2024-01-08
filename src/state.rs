use std::any::{Any, TypeId};

use crate::{entity::EntityID, component::ComponentList};

pub struct State {
    entities: Vec<EntityID>,
    register: Vec<ComponentList>,
    next_id: u32,
    exit: bool
}

impl State {
    pub fn new() -> State {
        State { 
            entities: vec![],
            register: vec![],
            next_id: 0,
            exit: false
        }
    }
    pub fn create_entity(&mut self) -> EntityID {
        let id = self.next_id;
        self.next_id += 1;
        let entity = EntityID { id };
        self.entities.push(entity);
        entity
    }
    pub fn delete_entity(&mut self, entity: &EntityID) {
        if !self.entities.contains(&entity) {
            panic!("tried to delete an entity that doesn't exist");
        }
        for i in 0..self.entities.len() {
            if self.entities[i] == *entity {
                self.entities.remove(i);
            }
        }
    }
    pub fn exit(&mut self) { self.exit = true }
    pub fn exiting(&self) -> bool { self.exit == true }
    fn find_register<C: Any>(&self) -> Option<usize> {
        let type_id = TypeId::of::<C>();
        for i in self.register.iter().enumerate() {
            if i.1.stored_type_id() == type_id {
                return Some(i.0);
            }
        }
        return None;
    }
    pub fn get_component<C: Any>(&self, entity: &EntityID) -> Option<&C> {
        let list = self.find_register::<C>();
        return match list {
            Some(index) => {
                Some(self.register[index].get::<C>(entity))
            }
            None => {
                None
            }
        }
    }
    pub fn get_mut_component<C: Any>(&mut self, entity: &EntityID) -> Option<&mut C> {
        let list = self.find_register::<C>();
        return match list {
            Some(index) => {
                Some(self.register[index].get_mut::<C>(entity))
            }
            None => {
                None
            }
        }
    }
    pub fn insert_component<C: Any>(&mut self, component: C, entity: &EntityID) {
        let list = self.find_register::<C>();
        match list {
            Some(index) => {
                self.register[index].insert(component, entity);
            }
            None => {
                self.register.push(ComponentList::new::<C>());
                self.register.last_mut().unwrap().insert(component, entity);
            }
        }
    }
    pub fn remove_component<C: Any>(&mut self, entity: &EntityID) {
        let list = self.find_register::<C>();
        match list {
            Some(index) => {
                self.register[index].remove(entity);
            }
            None => {
                panic!("tried to remove a component from an entity that didn't have it");
            }
        }
    }
    pub fn get_owners<C: Any>(&self) -> &Vec<EntityID> {
        let list = self.find_register::<C>();
        match list {
            Some(index) => {
                return self.register[index].get_owners()
            }
            None => {
                panic!("tried to remove a component from an entity that didn't have it");
            }
        }
    }
}