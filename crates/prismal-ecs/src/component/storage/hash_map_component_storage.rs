use std::collections::HashMap;

use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::string::key::KString;
use prismal_utils::sync::Mutex;

use crate::component::storage::component_storage::ComponentStorage;
use crate::component::ComponentKey;
use crate::entity::Entity;

pub struct HashMapComponentStorage<T: ComponentKey + Clone> {
    map: Mutex<HashMap<Entity, T, IntHasherBuilder>>,
}

impl<T: ComponentKey + Clone> ComponentStorage for HashMapComponentStorage<T> {
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

    fn remove_entity_component(&self, entity: Entity, component_key: KString) {
        let mut map = self.map.lock();
        match map.entry(entity) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                if component_key == entry.get().component_key() {
                    entry.remove();
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {}
        }
    }

    fn get(&self, entity: Entity) -> Self::IntoIter {
        let map = self.map.lock();
        map.get(&entity).map(Clone::clone)
    }

    fn len(&self) -> usize {
        self.map.lock().len()
    }
}
