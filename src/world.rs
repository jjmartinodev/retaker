use std::any::{Any, TypeId};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

use hashbrown::HashMap;
use parking_lot::lock_api::{RwLockReadGuard, RwLockWriteGuard};
use parking_lot::{RawRwLock, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(u64);

impl From<u64> for EntityId {
    fn from(val: u64) -> Self {
        EntityId(val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(u64);

impl From<u64> for ResourceId {
    fn from(val: u64) -> Self {
        ResourceId(val)
    }
}

pub struct World {
    component_table: HashMap<TypeId, Box<dyn TypeErasedListTrait>>,
    resource_table: HashMap<TypeId, Box<dyn Any>>,
    next_id: u64,
}

pub struct ComponentList<T: Any> {
    components: RwLock<HashMap<EntityId, T>>,
}

pub struct Resource<T: Any> {
    resource: RwLock<T>,
}

pub struct ComponentListRef<'a, T: Any> {
    pub(crate) lock: RwLockReadGuard<'a, RawRwLock, HashMap<EntityId, T>>,
}

pub struct ComponentListMut<'a, T: Any> {
    pub(crate) lock: RwLockWriteGuard<'a, RawRwLock, HashMap<EntityId, T>>,
}

pub struct ResourceRef<'a, T: Any> {
    lock: RwLockReadGuard<'a, RawRwLock, T>,
}

pub struct ResourceMut<'a, T: Any> {
    lock: RwLockWriteGuard<'a, RawRwLock, T>,
}

pub struct QueriedEntities<T: Any> {
    entities: Vec<EntityId>,
    phantom: PhantomData<T>,
}

pub struct Query<'a, T: Any> {
    entities: QueriedEntities<T>,
    
}

trait TypeErasedListTrait {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> TypeErasedListTrait for ComponentList<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl World {
    pub fn new() -> World {
        World {
            component_table: HashMap::new(),
            resource_table: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn component_list_ref<'a, T: Any>(&'a self) -> Option<ComponentListRef<'a, T>> {
        if let Some(list) = self.component_table.get(&TypeId::of::<T>()) {
            let list = list.as_any().downcast_ref::<ComponentList<T>>().unwrap();
            Some(ComponentListRef {
                lock: list.components.read(),
            })
        } else {
            None
        }
    }
    pub fn component_list_mut<'a, T: Any>(&'a self) -> Option<ComponentListMut<'a, T>> {
        if let Some(list) = self.component_table.get(&TypeId::of::<T>()) {
            let list = list.as_any().downcast_ref::<ComponentList<T>>().unwrap();
            Some(ComponentListMut {
                lock: list.components.write(),
            })
        } else {
            None
        }
    }
    pub fn resource_ref<'a, T: Any>(&'a self) -> Option<ResourceRef<'a, T>> {
        if let Some(resource) = self.component_table.get(&TypeId::of::<T>()) {
            let lock = resource.as_any().downcast_ref::<Resource<T>>().unwrap();
            Some(ResourceRef {
                lock: lock.resource.read(),
            })
        } else {
            None
        }
    }
    pub fn resource_mut<'a, T: Any>(&'a self) -> Option<ResourceMut<'a, T>> {
        if let Some(resource) = self.component_table.get(&TypeId::of::<T>()) {
            let lock = resource.as_any().downcast_ref::<Resource<T>>().unwrap();
            Some(ResourceMut {
                lock: lock.resource.write(),
            })
        } else {
            None
        }
    }
    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        id.into()
    }
    pub fn insert<T: Any>(&mut self, entity: &EntityId, component: T) -> Option<T> {
        if let Some(mut list) = self.component_list_mut() {
            return list.lock.insert(entity.clone(), component);
        }

        let mut components = HashMap::new();

        components.insert(entity.clone(), component);

        self.component_table.insert(
            TypeId::of::<T>(),
            Box::new(ComponentList::<T> {
                components: RwLock::new(components),
            }),
        );

        None
    }
    pub fn remove<T: Any>(&self, entity: &EntityId) -> Option<T> {
        if let Some(mut list) = self.component_list_mut::<T>() {
            list.lock.remove(entity)
        } else {
            None
        }
    }
}

impl<T: Any> IntoIterator for QueriedEntities<T> {
    type IntoIter = IntoIter<EntityId>;
    type Item = EntityId;
    fn into_iter(self) -> Self::IntoIter {
        self.entities.into_iter()
    }
}

impl<'a, T: Any> ComponentListRef<'a, T> {
    pub fn query(&self) -> QueriedEntities<T> {
        QueriedEntities {
            entities: self.lock.keys().cloned().collect(),
            phantom: PhantomData,
        }
    }
    pub fn with<Other: Any>(&self, mut query: QueriedEntities<Other>) -> QueriedEntities<Other> {
        query.entities = query
            .entities
            .into_iter()
            .filter(|e| self.lock.contains_key(e))
            .collect();
        query
    }
    pub fn without<Other: Any>(&self, mut query: QueriedEntities<Other>) -> QueriedEntities<Other> {
        query.entities = query
            .entities
            .into_iter()
            .filter(|e| !self.lock.contains_key(e))
            .collect();
        query
    }
    pub fn get(&self, entity: &EntityId) -> Option<&T> {
        self.lock.get(entity)
    }
}

impl<'a, T: Any> Deref for ResourceRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a, T: Any> Deref for ResourceMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a, T: Any> DerefMut for ResourceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.lock.deref_mut()
    }
}

impl<'a, T: Any> ComponentListMut<'a, T> {
    pub fn query(&self) -> QueriedEntities<T> {
        QueriedEntities {
            entities: self.lock.keys().cloned().collect(),
            phantom: PhantomData,
        }
    }
    pub fn with<Other: Any>(&self, mut query: QueriedEntities<Other>) -> QueriedEntities<Other> {
        query.entities = query
            .entities
            .into_iter()
            .filter(|e| self.lock.contains_key(e))
            .collect();
        query
    }
    pub fn without<Other: Any>(&self, mut query: QueriedEntities<Other>) -> QueriedEntities<Other> {
        query.entities = query
            .entities
            .into_iter()
            .filter(|e| !self.lock.contains_key(e))
            .collect();
        query
    }
    pub fn get(&self, entity: &EntityId) -> Option<&T> {
        self.lock.get(entity)
    }
    pub fn get_mut(&mut self, entity: &EntityId) -> Option<&mut T> {
        self.lock.get_mut(entity)
    }
    pub fn get_many_mut<const N: usize>(
        &mut self,
        entities: [&EntityId; N],
    ) -> Option<[&mut T; N]> {
        self.lock.get_many_mut(entities)
    }
}

#[test]
fn insert_remove_set_get() {
    type A = u32;
    type B = i32;

    let mut world = World::new();

    let alice = world.create_entity();
    let bob = world.create_entity();

    world.insert::<A>(&alice, 1);
    world.insert::<A>(&bob, 2);

    world.insert::<B>(&alice, 2);
    world.insert::<B>(&bob, 1);

    {
        let list = world.component_list_ref::<A>().unwrap();

        if let (Some(alice_val), Some(bob_val)) = (list.get(&alice), list.get(&bob)) {
            assert_eq!(
                (*alice_val, *bob_val),
                (1, 2),
                "testing correct component get from world, alice: {} bob: {}",
                *alice_val,
                *bob_val
            );
        }
    }

    {
        let mut list = world.component_list_mut::<B>().unwrap();

        let [alice_val, bob_val] = list.get_many_mut([&alice, &bob]).unwrap();
        *alice_val += *bob_val;
        *bob_val += *alice_val;
        assert_eq!(
            (*alice_val, *bob_val),
            (3, 4),
            "testing correct component get from world, alice: {} bob: {}",
            *alice_val,
            *bob_val
        );
    }

    assert_eq!(world.remove::<B>(&alice), Some(3));
    assert_eq!(world.remove::<B>(&bob), Some(4));

    assert_eq!(world.remove::<A>(&alice), Some(1));
    assert_eq!(world.remove::<A>(&bob), Some(2));
}

#[test]
fn query_with_without() {
    struct A;
    struct B;

    let mut world = World::new();

    let alice = world.create_entity();
    let bob = world.create_entity();
    let casie = world.create_entity();
    let daniel = world.create_entity();

    world.insert(&alice, A);
    world.insert(&bob, A);
    world.insert(&casie, A);

    world.insert(&alice, B);
    world.insert(&bob, B);
    world.insert(&daniel, B);

    let list_a = world.component_list_ref::<A>().unwrap();
    let list_b = world.component_list_ref::<B>().unwrap();

    for entity in list_b.with(list_a.query()) {
        assert!(
            alice == entity || bob == entity,
            "incorrectly queried \"with\"!"
        );
    }

    for entity in list_a.with(list_b.query()) {
        assert!(
            alice == entity || bob == entity,
            "incorrectly queried \"with\"!"
        );
    }

    for entity in list_b.without(list_a.query()) {
        assert!(casie == entity, "incorrectly queried \"without\"!");
    }

    for entity in list_a.without(list_b.query()) {
        assert!(daniel == entity, "incorrectly queried \"without\"!");
    }
}
