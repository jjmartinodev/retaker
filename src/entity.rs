use std::any::Any;

use crate::{aoe::Entity, world::EntityStorage};

pub type EntityId = u32;

pub struct EntityRef<'a> {
    pub(crate) storage: &'a EntityStorage,
    pub(crate) id: EntityId,
    pub(crate) entity: Option<&'a Entity>
}

pub struct EntityMut<'a> {
    pub(crate) storage: &'a mut EntityStorage,
    pub(crate) id: EntityId,
    pub(crate) entity: Option<&'a mut Entity>
}

impl<'a> EntityRef<'a> {
    pub fn component<C: Any>(&self) -> Option<&C> {
        if let Some(entity) = self.entity.as_ref() {
            entity.components.get()
        } else {
            match &(*self.storage) {
                EntityStorage::AOC(aoc) => {
                    aoc.get_component(&self.id)
                }
                EntityStorage::AOE(aoe) => {
                    aoe.component(&self.id)
                }
            }
        }
    }
}

impl<'a> EntityMut<'a> {
    pub fn component<C: Any>(&self) -> Option<&C> {
        if let Some(entity) = self.entity.as_ref() {
            entity.components.get()
        } else {
            match &(*self.storage) {
                EntityStorage::AOC(aoc) => {
                    aoc.get_component(&self.id)
                }
                EntityStorage::AOE(aoe) => {
                    aoe.component(&self.id)
                }
            }
        }
    }
    pub fn mut_component<C: Any>(&mut self) -> Option<&mut C> {
        if let Some(entity) = self.entity.as_mut() {
            entity.components.get_mut()
        } else {
            match self.storage {
                EntityStorage::AOC(aoc) => {
                    aoc.get_mut_component(&self.id)
                }
                EntityStorage::AOE(aoe) => {
                    aoe.mut_component(&self.id)
                }
            }
        }
    }
}