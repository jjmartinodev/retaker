use std::any::{Any, TypeId};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockUpgradableReadGuard, RwLockWriteGuard};

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

pub struct LockedWorld {
    world: RwLock<World>,
}

pub struct World {
    component_table: HashMap<TypeId, Box<dyn TypeErasedListTrait>>,
    resource_table: HashMap<TypeId, Box<dyn Any>>,
    next_entity_id: Mutex<u64>,
    next_resource_id: Mutex<u64>,
}

unsafe impl Sync for World {}
unsafe impl Send for World {}

pub struct ComponentList<T: Any + Send + Sync> {
    components: RwLock<HashMap<EntityId, T>>,
}

pub struct Resource<T: Any + Send + Sync> {
    resource: RwLock<T>,
}

pub struct ComponentListRef<'a, T: Any + Send + Sync> {
    pub(crate) lock: RwLockReadGuard<'a, HashMap<EntityId, T>>,
}

pub struct ComponentListMut<'a, T: Any + Send + Sync> {
    pub(crate) lock: RwLockWriteGuard<'a, HashMap<EntityId, T>>,
}

pub struct ResourceRef<'a, T: Any + Send + Sync> {
    lock: RwLockReadGuard<'a, T>,
}

pub struct ResourceMut<'a, T: Any + Send + Sync> {
    lock: RwLockWriteGuard<'a, T>,
}

#[derive(Clone)]
pub struct QueriedEntities {
    entities: Vec<EntityId>,
}

trait TypeErasedListTrait {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Send + Sync> TypeErasedListTrait for ComponentList<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LockedWorld {
    pub fn new() -> LockedWorld {
        LockedWorld {
            world: RwLock::new(World::new()),
        }
    }
    pub fn lock_shared(&self) -> ReaderWorldGuard {
        ReaderWorldGuard {
            lock: self.world.read(),
        }
    }
    pub fn lock_exclusive(&self) -> WriterWorldGuard {
        WriterWorldGuard {
            lock: self.world.write(),
        }
    }
    pub fn lock_upgradable(&self) -> UpgradableReaderWorldGuard {
        UpgradableReaderWorldGuard {
            lock: RwLockWriteGuard::downgrade_to_upgradable(self.world.write()),
        }
    }
}

pub struct WriterWorldGuard<'a> {
    lock: RwLockWriteGuard<'a, World>,
}

pub struct ReaderWorldGuard<'a> {
    lock: RwLockReadGuard<'a, World>,
}

pub struct UpgradableReaderWorldGuard<'a> {
    lock: RwLockUpgradableReadGuard<'a, World>,
}

impl<'a> Deref for ReaderWorldGuard<'a> {
    type Target = World;
    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a> Deref for UpgradableReaderWorldGuard<'a> {
    type Target = World;
    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a> Deref for WriterWorldGuard<'a> {
    type Target = World;
    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a> DerefMut for WriterWorldGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.lock.deref_mut()
    }
}

impl<'a> UpgradableReaderWorldGuard<'a> {
    pub fn upgrade(self) -> WriterWorldGuard<'a> {
        WriterWorldGuard {
            lock: RwLockUpgradableReadGuard::upgrade(self.lock),
        }
    }
}

impl World {
    pub fn new() -> World {
        World {
            component_table: HashMap::new(),
            resource_table: HashMap::new(),
            next_entity_id: Mutex::new(0),
            next_resource_id: Mutex::new(0),
        }
    }
    pub fn component_list_ref<'a, T: Any + Send + Sync>(
        &'a self,
    ) -> Option<ComponentListRef<'a, T>> {
        if let Some(list) = self.component_table.get(&TypeId::of::<T>()) {
            let list = list.as_any().downcast_ref::<ComponentList<T>>().unwrap();
            Some(ComponentListRef {
                lock: list.components.read(),
            })
        } else {
            None
        }
    }
    pub fn component_list_mut<'a, T: Any + Send + Sync>(
        &'a self,
    ) -> Option<ComponentListMut<'a, T>> {
        if let Some(list) = self.component_table.get(&TypeId::of::<T>()) {
            let list = list.as_any().downcast_ref::<ComponentList<T>>().unwrap();
            Some(ComponentListMut {
                lock: list.components.write(),
            })
        } else {
            None
        }
    }
    pub fn resource_ref<'a, T: Any + Send + Sync>(&'a self) -> Option<ResourceRef<'a, T>> {
        if let Some(resource) = self.resource_table.get(&TypeId::of::<T>()) {
            let lock = resource.downcast_ref::<Resource<T>>().unwrap();
            Some(ResourceRef {
                lock: lock.resource.read(),
            })
        } else {
            None
        }
    }
    pub fn resource_mut<'a, T: Any + Send + Sync>(&'a self) -> Option<ResourceMut<'a, T>> {
        if let Some(resource) = self.resource_table.get(&TypeId::of::<T>()) {
            let lock = resource.downcast_ref::<Resource<T>>().unwrap();
            Some(ResourceMut {
                lock: lock.resource.write(),
            })
        } else {
            None
        }
    }
    pub fn create_resource<T: Any + Send + Sync>(&mut self, resource: T) -> ResourceId {
        let mut id_guard = self.next_resource_id.lock();
        let id = id_guard.clone();
        *id_guard += 1;
        self.resource_table.insert(
            TypeId::of::<T>(),
            Box::new(Resource {
                resource: RwLock::new(resource),
            }),
        );
        id.into()
    }
    pub fn create_entity(&self) -> EntityId {
        let mut id_guard = self.next_entity_id.lock();
        let id = id_guard.clone();
        *id_guard += 1;
        id.into()
    }
    pub fn insert<T: Any + Send + Sync>(&mut self, entity: &EntityId, component: T) -> Option<T> {
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
    pub fn remove<T: Any + Send + Sync>(&self, entity: &EntityId) -> Option<T> {
        if let Some(mut list) = self.component_list_mut::<T>() {
            list.lock.remove(entity)
        } else {
            None
        }
    }
}

impl IntoIterator for QueriedEntities {
    type IntoIter = IntoIter<EntityId>;
    type Item = EntityId;
    fn into_iter(self) -> Self::IntoIter {
        self.entities.into_iter()
    }
}

impl<'a, T: Any + Send + Sync> ComponentListRef<'a, T> {
    pub fn query(&self) -> QueriedEntities {
        QueriedEntities {
            entities: self.lock.keys().cloned().collect(),
        }
    }
    pub fn with(&self, mut query: QueriedEntities) -> QueriedEntities {
        query.entities = query
            .entities
            .into_iter()
            .filter(|e| self.lock.contains_key(e))
            .collect();
        query
    }
    pub fn without(&self, mut query: QueriedEntities) -> QueriedEntities {
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

impl<'a, T: Any + Send + Sync> Deref for ResourceRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a, T: Any + Send + Sync> Deref for ResourceMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.lock.deref()
    }
}

impl<'a, T: Any + Send + Sync> DerefMut for ResourceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.lock.deref_mut()
    }
}

impl<'a, T: Any + Send + Sync> ComponentListMut<'a, T> {
    pub fn query(&self) -> QueriedEntities {
        QueriedEntities {
            entities: self.lock.keys().cloned().collect(),
        }
    }
    pub fn with(&self, mut query: QueriedEntities) -> QueriedEntities {
        query.entities = query
            .entities
            .into_iter()
            .filter(|e| self.lock.contains_key(e))
            .collect();
        query
    }
    pub fn without(&self, mut query: QueriedEntities) -> QueriedEntities {
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
    pub fn clear(&mut self) {
        self.lock.clear();
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

#[test]
fn query_resource() {
    use std::time::Instant;
    struct Timer {
        last_instant: Instant,
        interval_ms: u64,
    }

    let mut world = World::new();
    world.create_resource(Timer {
        last_instant: Instant::now(),
        interval_ms: 100,
    });

    let mut n = 0;

    loop {
        if n > 3 {
            break;
        }
        let mut timer = world.resource_mut::<Timer>().unwrap();
        if timer.last_instant.elapsed().as_millis() > timer.interval_ms as u128 {
            timer.last_instant = Instant::now();
            n += 1;
        }
    }
}
