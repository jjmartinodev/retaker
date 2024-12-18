use std::any::{Any, TypeId};

use hashbrown::HashMap;

use crate::component::{ComponentList, World};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct EntityId(u32);

pub struct ComponentRef {
    component: *const dyn Any,
    guard: *const dyn ComponentList,
}

pub struct EntityRef {
    pub(crate)references: HashMap<TypeId, ComponentRef>,
    entity: EntityId,
}

pub struct ComponentMut {
    component: *mut dyn Any,
    guard: *const dyn ComponentList,
}

pub struct EntityMut {
    pub(crate)references: HashMap<TypeId, ComponentMut>,
    entity: EntityId,
}

impl EntityId {
    pub fn new(id: u32) -> EntityId {
        EntityId(id)
    }
    pub fn id(&self) -> u32 {
        self.0
    }
}

impl EntityRef {
    pub fn new(world: &World, entity: &EntityId) -> EntityRef {
        let mut references = HashMap::new();
        world
            .type_lists
            .read()
            .iter()
            .filter(|(_, list)| list.contains(entity))
            .map(|(id, list)| {
                let lock = unsafe { list.lock_ref(entity) };
                references.insert(
                    *id,
                    ComponentRef {
                        guard: lock.0,
                        component: lock.1,
                    },
                )
            })
            .for_each(|_| {});
        EntityRef {
            references,
            entity: entity.clone(),
        }
    }
    pub fn id(&self) -> EntityId {
        self.entity.clone()
    }
    pub unsafe fn recycle(mut self, entity: &EntityId) -> EntityRef {
        self.references.iter_mut().for_each(|reference| {
            reference.1.component = unsafe { reference.1.guard.as_ref() }.unwrap().get(entity).unwrap();
        });
        self
    }
    pub fn component<T: Any>(&self) -> Option<&T> {
        if let Some(list) = self.references.get(&TypeId::of::<T>()) {
            unsafe { list.component.as_ref() }
                .unwrap()
                .downcast_ref::<T>()
        } else {
            None
        }
    }
}

impl EntityMut {
    pub fn new(world: &World, entity: &EntityId) -> EntityMut {
        let mut references = HashMap::new();
        world
            .type_lists
            .read()
            .iter()
            .filter(|(_, list)| list.contains(entity))
            .map(|(id, list)| {
                let lock = unsafe { list.lock_mut(entity) };
                references.insert(
                    *id,
                    ComponentMut {
                        guard: lock.0,
                        component: lock.1,
                    },
                )
            })
            .for_each(|_| {});
        EntityMut {
            references,
            entity: entity.clone(),
        }
    }
    pub unsafe fn recycle(mut self, entity: &EntityId) -> EntityMut {
        self.references.iter_mut().for_each(|reference| {
            reference.1.component = unsafe { reference.1.guard.as_ref() }
                .unwrap()
                .get_mut(entity).unwrap();
        });
        self
    }
    pub fn id(&self) -> EntityId {
        self.entity.clone()
    }
    pub fn component<T: Any>(&self) -> Option<&T> {
        if let Some(list) = self.references.get(&TypeId::of::<T>()) {
            unsafe { list.component.as_ref() }
                .unwrap()
                .downcast_ref::<T>()
        } else {
            None
        }
    }
    pub fn mut_component<T: Any>(&self) -> Option<&mut T> {
        if let Some(list) = self.references.get(&TypeId::of::<T>()) {
            unsafe { list.component.as_mut() }
                .unwrap()
                .downcast_mut::<T>()
        } else {
            None
        }
    }
}

impl Drop for EntityRef {
    fn drop(&mut self) {
        self.references
            .iter()
            .map(|(_, list)| {
                unsafe { list.guard.as_ref().unwrap().unlock_ref() };
            })
            .for_each(|_| {});
    }
}

impl Drop for EntityMut {
    fn drop(&mut self) {
        self.references
            .iter()
            .map(|(_, list)| {
                unsafe { list.guard.as_ref().unwrap().unlock_mut() };
            })
            .for_each(|_| {});
    }
}
