use std::collections::HashSet;

use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::prelude::InteriorMut;
use prismal_utils::sync::Mutex;

use crate::entity::Entity;

#[derive(Debug)]
pub struct World {
    live_entities: Mutex<HashSet<Entity, IntHasherBuilder>>,
    next_index: Mutex<u64>,
}

impl World {
    pub fn new() -> Self {
        Self {
            live_entities: Mutex::new(HashSet::with_hasher(IntHasherBuilder)),
            next_index: Mutex::new(1),
        }
    }

    pub fn spawn_entity(&self) -> Entity {
        let mut next_index_ref = self.next_index.borrow_int_mut().unwrap();
        let index = *next_index_ref;
        *next_index_ref += 1;

        let entity = Entity::from_index(index);
        let mut entities_ref = self.live_entities.borrow_int_mut().unwrap();
        entities_ref.insert(entity);

        entity
    }

    pub fn kill_entity(&self, entity: Entity) {
        let mut entities_ref = self.live_entities.borrow_int_mut().unwrap();
        entities_ref.remove(&entity);
    }
}
