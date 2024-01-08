use std::any::{TypeId, Any};

use anylist::AnyList;

use crate::entity::EntityID;

pub struct ComponentList {
    typeid: TypeId,
    list: AnyList,
    owners: Vec<EntityID>
}

impl ComponentList {
    pub fn new<C: Any>() -> ComponentList {
        ComponentList {
            typeid: TypeId::of::<C>(),
            list: AnyList::new::<C>(),
            owners: vec![]
        }
    }
    fn find_owner(&self, entity: &EntityID) -> Option<usize> {
        for owner in self.owners.iter().enumerate() {
            if owner.1 == entity {
                return Some(owner.0);
            }
        }
        None
    }
    pub fn get<C: Any>(&self, entity: &EntityID) -> &C {
        let index = self.find_owner(entity);
        match index {
            Some(index) => {
                return self.list.index(index);
            }
            None => {
                panic!("couldn't get component from entity");
            }
        }
    }
    pub fn get_mut<C: Any>(&mut self, entity: &EntityID) -> &mut C {
        let index = self.find_owner(entity);
        match index {
            Some(index) => {
                return self.list.index_mut(index);
            }
            None => {
                panic!("couldn't get mutable component from entity");
            }
        }
    }
    pub fn insert<C: Any>(&mut self, component: C, entity: &EntityID) {
        self.list.push(component);
        self.owners.push(*entity);
    }
    pub fn no_panic_remove(&mut self, entity: &EntityID) {
        let index = self.find_owner(entity);
        match index {
            Some(index) => {
                self.list.remove(index);
                self.owners.remove(index);
            }
            None => ()
        }
    }
    pub fn remove(&mut self, entity: &EntityID) {
        let index = self.find_owner(entity);
        match index {
            Some(index) => {
                self.list.remove(index);
                self.owners.remove(index);
            }
            None => {
                panic!("couldn't remove component from entity");
            }
        }
    }
    pub const fn stored_type_id(&self) -> TypeId {
        self.typeid
    }
    pub fn get_owners(&self) -> &Vec<EntityID> {
        &self.owners
    }
}