use std::any::{TypeId, Any};

use anylist::AnyList;

use crate::entity::EntityID;

#[derive(Debug)]
pub enum ComponentError {
    NoMatchingOwner
}

pub struct ComponentList {
    components: AnyList,
    owners: Vec<EntityID>,
    pub(crate) typeid: TypeId
}

impl ComponentList {
    pub fn new<C: Any>() -> ComponentList {
        ComponentList {
            components: AnyList::new::<C>(),
            owners: vec![],
            typeid: TypeId::of::<C>()
        }
    }
    pub fn add<C: Any>(&mut self, owner: EntityID, component: C) {
        self.components.push::<C>(component);
        self.owners.push(owner);
    }
    pub fn owners(&self) -> &Vec<EntityID> {
        &self.owners
    }
    pub fn get<C: Any>(&self, owner: EntityID) -> Result<&C, ComponentError> {
        for i in 0..self.owners.len() {
            if self.owners[i] == owner {
                return Ok(self.components.index(i));
            }
        }
        Err(ComponentError::NoMatchingOwner)
    }
}