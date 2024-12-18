use std::{
    any::{Any, TypeId},
    sync::atomic::AtomicU32,
};

use hashbrown::HashMap;
use parking_lot::{lock_api::RawRwLock, RwLock};

use crate::entity::{EntityId, EntityMut, EntityRef};

pub struct Components<T: Any + Send + Sync> {
    pub(crate) components: RwLock<HashMap<EntityId, T>>,
}

pub struct World {
    pub(crate) type_lists: RwLock<HashMap<TypeId, Box<dyn ComponentList>>>,
}

pub struct EntityIdGenerator {
    next_id: AtomicU32,
}

pub trait ComponentList: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;

    fn remove_untyped(&self, entity: &EntityId);
    fn contains(&self, entity: &EntityId) -> bool;

    unsafe fn lock_ref(&self, entity: &EntityId) -> (*const dyn ComponentList, *const dyn Any);
    unsafe fn unlock_ref(&self);

    unsafe fn lock_mut(&self, entity: &EntityId) -> (*const dyn ComponentList, *mut dyn Any);
    unsafe fn unlock_mut(&self);

    unsafe fn get(&self, entity: &EntityId) -> Option<*const dyn Any>;
    unsafe fn get_mut(&self, entity: &EntityId) -> Option<*mut dyn Any>;
}

impl<T: Any + Send + Sync> ComponentList for Components<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    fn remove_untyped(&self, entity: &EntityId) {
        self.components.write().remove(&entity);
    }
    fn contains(&self, entity: &EntityId) -> bool {
        self.components.read().contains_key(entity)
    }
    unsafe fn lock_ref(&self, entity: &EntityId) -> (*const dyn ComponentList, *const dyn Any) {
        self.components.raw().lock_shared();
        (
            self as *const dyn ComponentList,
            self.components.read().get(entity).unwrap() as *const dyn Any,
        )
    }
    unsafe fn unlock_ref(&self) {
        self.components.raw().unlock_shared();
    }

    unsafe fn lock_mut(&self, entity: &EntityId) -> (*const dyn ComponentList, *mut dyn Any) {
        let component = self.components.write().get_mut(entity).unwrap() as *mut dyn Any;
        self.components.raw().lock_shared();
        (self as *const dyn ComponentList, component)
    }
    unsafe fn unlock_mut(&self) {
        self.components.raw().unlock_shared();
    }
    unsafe fn get(&self, entity: &EntityId) -> Option<*const dyn Any> {
        let val = unsafe { self.components.data_ptr().as_ref() }
            .unwrap()
            .get(entity);
        if let Some(val) = val {
            Some(val as *const dyn Any)
        } else {
            None
        }
    }
    unsafe fn get_mut(&self, entity: &EntityId) -> Option<*mut dyn Any> {
        let val = unsafe { self.components.data_ptr().as_mut() }
            .unwrap()
            .get_mut(entity);
        if let Some(val) = val {
            Some(val as *mut dyn Any)
        } else {
            None
        }
    }
}

impl World {
    pub fn new() -> World {
        World {
            type_lists: RwLock::new(HashMap::new()),
        }
    }
    fn component_list<T: Any + Send + Sync, F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Components<T>) -> R,
    {
        if let Some(list) = self.type_lists.read().get(&TypeId::of::<T>()) {
            Some(f(list.as_any().downcast_ref::<Components<T>>().unwrap()))
        } else {
            None
        }
    }
    pub fn is_registered<T: Any + Send + Sync>(&self) -> bool {
        self.type_lists.read().contains_key(&TypeId::of::<T>())
    }
    pub fn register_type<T: Any + Send + Sync>(&self) {
        self.type_lists
            .write()
            .insert(TypeId::of::<T>(), Box::new(Components::<T>::new()));
    }
    pub fn insert<T: Any + Send + Sync>(&self, entity: &EntityId, component: T) -> Option<T> {
        if !self.is_registered::<T>() {
            self.register_type::<T>();
        }
        self.component_list::<T, _, Option<T>>(|list| {
            list.components.write().insert(entity.clone(), component)
        })
        .unwrap()
    }
    pub fn insert_many<E, T: Any + Send + Sync>(&self, entities: E)
    where
        E: IntoIterator<Item = (EntityId, T)>,
    {
        if !self.is_registered::<T>() {
            self.register_type::<T>();
        }
        let entity_iter = entities.into_iter();
        self.component_list::<T, _, _>(|list| {
            let mut comps = list.components.write();
            comps.reserve(entity_iter.size_hint().1.unwrap());
            entity_iter.for_each(|(e, c)| {
                comps.insert(e.clone(), c);
            });
        });
    }
    pub fn remove<T: Any + Send + Sync>(&self, entity: &EntityId) -> Option<T> {
        if !self.is_registered::<T>() {
            return None;
        }
        self.component_list::<T, _, Option<T>>(|list| list.components.write().remove(&entity))
            .unwrap()
    }
    pub fn delete_entity(&self, entity: &EntityId) {
        for list in self.type_lists.read().values() {
            if list.contains(entity) {
                list.remove_untyped(entity);
            }
        }
    }
    pub fn query<T: Any + Send + Sync, F: Any + Send + Sync, M>(&self, f: M)
    where
        M: Fn(&EntityRef) -> (),
    {
        let owners = self
            .component_list::<T, _, _>(|list| {
                list.components.read().keys().cloned().collect::<Vec<_>>()
            })
            .unwrap();
        let excluded = self
            .component_list::<F, _, _>(|list| {
                list.components.read().keys().cloned().collect::<Vec<_>>()
            })
            .unwrap_or(vec![]);
        let mut outer_recycled: Option<EntityRef> = None;
        for i in 0..owners.len() {
            if excluded.contains(&owners[i]) {
                continue;
            }
            if let Some(recycled) = outer_recycled.take() {
                let reference = unsafe { recycled.recycle(&owners[i]) };
                f(&reference);
                outer_recycled = Some(reference);
            } else {
                let reference = EntityRef::new(&self, &owners[i]);
                f(&reference);
                outer_recycled = Some(reference);
            }
        }
    }
    pub fn query_mut<T: Any + Send + Sync, F: Any + Send + Sync, M>(&self, f: M)
    where
        M: Fn(&EntityMut) -> (),
    {
        let owners = self
            .component_list::<T, _, _>(|list| {
                list.components.read().keys().cloned().collect::<Vec<_>>()
            })
            .unwrap();
        let excluded = self
            .component_list::<F, _, _>(|list| {
                list.components.read().keys().cloned().collect::<Vec<_>>()
            })
            .unwrap_or(vec![]);
        let mut outer_recycled: Option<EntityMut> = None;
        for i in 0..owners.len() {
            if excluded.contains(&owners[i]) {
                continue;
            }
            if let Some(recycled) = outer_recycled.take() {
                let reference = unsafe { recycled.recycle(&owners[i]) };
                f(&reference);
                outer_recycled = Some(reference);
            } else {
                let reference = EntityMut::new(&self, &owners[i]);
                f(&reference);
                outer_recycled = Some(reference);
            }
        }
    }
    pub fn query_pairs_mut<T: Any + Send + Sync, F: Any + Send + Sync, M>(&self, f: M)
    where
        M: Fn(&(EntityMut, EntityMut)) -> (),
    {
        let owners = self
            .component_list::<T, _, _>(|list| {
                list.components.read().keys().cloned().collect::<Vec<_>>()
            })
            .unwrap();
        let excluded = self
            .component_list::<F, _, _>(|list| {
                list.components.read().keys().cloned().collect::<Vec<_>>()
            })
            .unwrap_or(vec![]);
        let mut outer_recycled: Option<(EntityMut, EntityMut)> = None;
        for i in 0..owners.len() {
            if excluded.contains(&owners[i]) {
                continue;
            }
            for j in 0..owners.len() {
                if excluded.contains(&owners[j]) {
                    continue;
                }
                if let Some(mut recycled) = outer_recycled.take() {
                    recycled.1 = unsafe { recycled.1.recycle(&owners[j]) };
                    f(&recycled);
                    outer_recycled = Some(recycled);
                } else {
                    let reference = (
                        EntityMut::new(&self, &owners[i]),
                        EntityMut::new(&self, &owners[j]),
                    );
                    f(&reference);
                    outer_recycled = Some(reference);
                }
            }
            if let Some(mut recycled) = outer_recycled.take() {
                recycled.0 = unsafe { recycled.0.recycle(&owners[i]) };
                outer_recycled = Some(recycled);
            }
        }
    }
}

impl EntityIdGenerator {
    pub fn new() -> EntityIdGenerator {
        EntityIdGenerator {
            next_id: AtomicU32::new(0),
        }
    }
    pub fn generate(&self) -> EntityId {
        let n = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        EntityId::new(n)
    }
}

impl<T: Any + Send + Sync> Components<T> {
    pub fn new() -> Components<T> {
        Components {
            components: RwLock::new(HashMap::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::component::EntityIdGenerator;

    #[test]
    fn test_world_query() {
        use super::World;
        let world = Arc::new(World::new());
        let entity_gen = Arc::new(EntityIdGenerator::new());
        let sara = entity_gen.generate();
        world.insert(&sara, String::from("Sara"));
        world.insert(&sara, 12u32);

        let alan = entity_gen.generate();
        world.insert(&alan, String::from("Alan"));
        world.insert(&alan, 14u32);
        world.insert(&alan, 181.0f32);

        let copy_a = world.clone();
        let a = std::thread::spawn(move || {
            println!("first!");
            copy_a.query_mut::<String, (), _>(|e| {
                let s = e.mut_component::<String>();
                let a = e.mut_component::<u32>();
                let l = e.mut_component::<f32>();
                if let Some(name) = s {
                    println!("{name}");
                    *name = name.to_uppercase();
                }
                if let Some(age) = a {
                    println!("{age}");
                    *age += 1;
                }
                if let Some(height) = l {
                    println!("{height}");
                    *height += 0.5;
                }
            });
        });
        let b = std::thread::spawn(move || {
            println!("second!");
            world.clone().query_pairs_mut::<String, (), _>(|(a,b)| {
                {
                    let s = b.component::<String>();
                    let a = b.component::<u32>();
                    let l = b.component::<f32>();
                    println!("{s:?} {a:?} {l:?}");
                }
                {
                    let s = a.component::<String>();
                    let c = a.component::<u32>();
                    let l = a.component::<f32>();
                    println!("{s:?} {c:?} {l:?}");
                }
            });
        });

        a.join().unwrap();
        b.join().unwrap();
    }
}
