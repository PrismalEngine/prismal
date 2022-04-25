use std::collections::hash_map::Entry;
use std::collections::HashMap;

use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::sync::Mutex;

use crate::component::storage::component_storage::ComponentStorage;
use crate::entity::Entity;

pub struct MultiHashMapComponentStorage<T: Send + Sync + Clone> {
    map: Mutex<HashMap<Entity, Vec<T>, IntHasherBuilder>>,
}

impl<T: Send + Sync + Clone> ComponentStorage for MultiHashMapComponentStorage<T> {
    type Stored = T;
    type IntoIter = Vec<T>;

    fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::default()),
        }
    }
    fn insert(&self, entity: Entity, component: Self::Stored) {
        let mut map = self.map.lock();
        match map.entry(entity) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(component);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![component]);
            }
        }
    }
    fn remove_entity(&self, entity: Entity) {
        let mut map = self.map.lock();
        map.remove(&entity);
    }
    fn get(&self, entity: Entity) -> Self::IntoIter {
        let map = self.map.lock();
        map.get(&entity).map(Clone::clone).unwrap_or_default()
    }
    fn len(&self) -> usize {
        self.map.lock().iter().map(|(_, v)| v.len()).sum()
    }
}
