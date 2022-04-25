use std::collections::HashMap;

use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::sync::Mutex;

use crate::component::storage::component_storage::ComponentStorage;
use crate::entity::Entity;

pub struct HashMapComponentStorage<T: Send + Sync> {
    map: Mutex<HashMap<Entity, T, IntHasherBuilder>>,
}

impl<T: Send + Sync + Clone> ComponentStorage for HashMapComponentStorage<T> {
    type Stored = T;
    type IntoIter = Option<T>;

    fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::default()),
        }
    }
    fn insert(&self, entity: Entity, component: Self::Stored) {
        let mut map = self.map.lock();
        map.insert(entity, component);
    }
    fn remove_entity(&self, entity: Entity) {
        let mut map = self.map.lock();
        map.remove(&entity);
    }

    fn get(&self, entity: Entity) -> Self::IntoIter {
        let map = self.map.lock();
        map.get(&entity).map(Clone::clone)
    }

    fn len(&self) -> usize {
        self.map.lock().len()
    }
}
