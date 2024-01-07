use std::any::{Any, TypeId, type_name};

use crate::{entity::EntityID, component::ComponentList};

pub struct State {
    entities: Vec<EntityID>,
    register: Vec<ComponentList>,
    next_id:u32,
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
        self.entities.push(EntityID { id });
        self.next_id += 1;
        EntityID { id }
    }
    pub fn exit(&mut self) { self.exit = true }
    pub fn exiting(&self) -> bool { self.exit }
    fn component_location<C: Any>(&self) -> Option<usize> {
        let typeid = TypeId::of::<C>();
        for i in 0..self.register.len() {
            if self.register[i].typeid == typeid {
                return Some(i);
            }
        }
        None
    }
    fn register_component<C: Any>(&mut self) {
        self.register.push(ComponentList::new::<C>());
    }
    pub fn add_component<C: Any>(&mut self, entity: EntityID, component: C) {
        match self.component_location::<C>() {
            Some(index) => {
                self.register[index].add(entity, component);
            }
            None => {
                self.register_component::<C>();
                self.add_component(entity, component);
            }
        }
    }
    pub fn get_owners<C: Any>(&self) -> &Vec<EntityID> {
        match self.component_location::<C>() {
            Some(index) => {
                return self.register[index].owners();
            }
            None => {
                panic!("couldn't find component {} in the registry", type_name::<C>());
            }
        }
    }
    pub fn get_component<C: Any>(&self, entity: EntityID) -> &C {
        match self.component_location::<C>() {
            Some(index) => {
                return self.register[index].get(entity).expect("failed getting component");
            }
            None => {
                panic!("couldn't find component {} in the registry", type_name::<C>());
            }
        }
    }
}