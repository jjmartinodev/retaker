use std::any::Any;

use crate::world::EntityStorage;

pub type EntityId = u32;

pub struct EntityRef<'a> {
    pub(crate) storage: &'a EntityStorage,
    pub(crate) id: EntityId
}

pub struct EntityMut<'a> {
    pub(crate) storage: &'a mut EntityStorage,
    pub(crate) id: EntityId
}

impl<'a> EntityRef<'a> {
    pub fn get_component<C: Any>(&self) -> Option<&C> {
        match self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.get_component(&self.id)
            }
            EntityStorage::AOE(aoe) => {
                aoe.get_component(&self.id)
            }
        }
    }
}

impl<'a> EntityMut<'a> {
    pub fn get_component<C: Any>(&self) -> Option<&C> {
        match &(*self.storage) {
            EntityStorage::AOC(aoc) => {
                aoc.get_component(&self.id)
            }
            EntityStorage::AOE(aoe) => {
                aoe.get_component(&self.id)
            }
        }
    }
    pub fn get_mut_component<C: Any>(&mut self) -> Option<&mut C> {
        match self.storage {
            EntityStorage::AOC(aoc) => {
                aoc.get_mut_component(&self.id)
            }
            EntityStorage::AOE(aoe) => {
                aoe.get_mut_component(&self.id)
            }
        }
    }
}