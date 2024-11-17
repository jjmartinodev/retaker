
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId {
    id: u32,
}

pub struct DefaultEntityIdGenerator {
    next_id: u32
}

impl EntityId {
    pub fn new(id: u32) -> EntityId {
        EntityId { id }
    }
    pub fn get(&self) -> u32 {
        self.id
    }
}

impl DefaultEntityIdGenerator {
    pub fn new() -> DefaultEntityIdGenerator {
        DefaultEntityIdGenerator {
            next_id: 1000
        }
    }
    pub fn generate(&mut self) -> EntityId {
        let entity = EntityId::new(self.next_id);
        self.next_id += 1;
        entity
    }
}

impl Default for DefaultEntityIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}