use std::{
    any::Any,
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use hashbrown::HashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{entity::EntityId, world::ComponentList};

pub struct ComponentReadGuard<'a> {
    access: RwLockReadGuard<'a, Box<dyn ComponentList>>,
}

pub struct ComponentWriteGuard<'a> {
    access: RwLockWriteGuard<'a, Box<dyn ComponentList>>,
}

pub struct ComponentRef<'a, C: Any> {
    access: ComponentReadGuard<'a>,
    phantom: PhantomData<C>,
    entity: EntityId,
}

pub struct ComponentMut<'a, C: Any> {
    access: ComponentWriteGuard<'a>,
    phantom: PhantomData<C>,
    entity: EntityId,
}

pub struct ManyComponentMut<'a, C: Any, const N: usize> {
    access: ComponentWriteGuard<'a>,
    phantom: PhantomData<C>,
    entities: [EntityId; N],
}

impl<'a> ComponentWriteGuard<'a> {
    pub fn new<C: Any>(
        access: &'a RwLock<Box<(dyn ComponentList + 'static)>>,
    ) -> ComponentWriteGuard<'a> {
        ComponentWriteGuard {
            access: access.write(),
        }
    }
}

impl<'a> ComponentReadGuard<'a> {
    pub fn new<C: Any>(
        access: &'a RwLock<Box<(dyn ComponentList + 'static)>>,
    ) -> ComponentReadGuard<'a> {
        ComponentReadGuard {
            access: access.read(),
        }
    }
}

impl<'a, C: Any> ComponentRef<'a, C> {
    pub fn new(list: ComponentReadGuard<'a>, entity: &EntityId) -> ComponentRef<'a, C> {
        ComponentRef {
            access: list,
            phantom: PhantomData,
            entity: *entity,
        }
    }
    pub fn drop(self) -> ComponentReadGuard<'a> {
        self.access
    }
}

impl<'a, C: Any> ComponentMut<'a, C> {
    pub fn new(list: ComponentWriteGuard<'a>, entity: &EntityId) -> ComponentMut<'a, C> {
        ComponentMut {
            access: list,
            phantom: PhantomData,
            entity: *entity,
        }
    }
    pub fn drop(self) -> ComponentWriteGuard<'a> {
        self.access
    }
}

impl<'a, C: Any, const N: usize> ManyComponentMut<'a, C, N> {
    pub fn new(
        list: ComponentWriteGuard<'a>,
        entities: [EntityId; N],
    ) -> ManyComponentMut<'a, C, N> {
        ManyComponentMut {
            access: list,
            phantom: PhantomData,
            entities,
        }
    }
    pub fn get(&mut self) -> [&mut C; N] {
        self.access
            .access
            .as_mut_any()
            .downcast_mut::<HashMap<EntityId, C>>()
            .unwrap()
            .get_many_mut(self.entities.each_ref())
            .unwrap()
    }
    pub fn drop(self) -> ComponentWriteGuard<'a> {
        self.access
    }
}

impl<C: Any> Borrow<C> for ComponentRef<'_, C> {
    fn borrow(&self) -> &C {
        self.access
            .access
            .as_any()
            .downcast_ref::<HashMap<EntityId, C>>()
            .unwrap()
            .get(&self.entity)
            .unwrap()
    }
}

impl<C: Any> BorrowMut<C> for ComponentMut<'_, C> {
    fn borrow_mut(&mut self) -> &mut C {
        self.access
            .access
            .as_mut_any()
            .downcast_mut::<HashMap<EntityId, C>>()
            .unwrap()
            .get_mut(&self.entity)
            .unwrap()
    }
}

impl<C: Any> Borrow<C> for ComponentMut<'_, C> {
    fn borrow(&self) -> &C {
        self.access
            .access
            .as_any()
            .downcast_ref::<HashMap<EntityId, C>>()
            .unwrap()
            .get(&self.entity)
            .unwrap()
    }
}

impl<C: Any> Deref for ComponentRef<'_, C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        self.access
            .access
            .as_any()
            .downcast_ref::<HashMap<EntityId, C>>()
            .unwrap()
            .get(&self.entity)
            .unwrap()
    }
}

impl<C: Any> Deref for ComponentMut<'_, C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        self.access
            .access
            .as_any()
            .downcast_ref::<HashMap<EntityId, C>>()
            .unwrap()
            .get(&self.entity)
            .unwrap()
    }
}

impl<C: Any> DerefMut for ComponentMut<'_, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.access
            .access
            .as_mut_any()
            .downcast_mut::<HashMap<EntityId, C>>()
            .unwrap()
            .get_mut(&self.entity)
            .unwrap()
    }
}
