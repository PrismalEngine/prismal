use std::collections::hash_map::Entry;
use std::collections::HashMap;

use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::prelude::InteriorMut;
use prismal_utils::shared::{rc_mut, RcMut};
use prismal_utils::string::key::KString;
use prismal_utils::sync::Mutex;

use crate::component::storage::component_storage::ComponentStorage;
use crate::component::ComponentKey;
use crate::entity::Entity;

pub struct MultiHashMapComponentStorage<T: ComponentKey + Clone> {
    map: Mutex<HashMap<Entity, Vec<RcMut<T>>, IntHasherBuilder>>,
}

impl<T: ComponentKey + Clone> ComponentStorage for MultiHashMapComponentStorage<T> {
    type Stored = T;
    type IntoIter = Vec<RcMut<T>>;

    fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::default()),
        }
    }
    fn insert(&self, entity: Entity, component: Self::Stored) {
        let mut map = self.map.lock();
        match map.entry(entity) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(rc_mut(component));
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![rc_mut(component)]);
            }
        }
    }
    fn remove_entity(&self, entity: Entity) {
        let mut map = self.map.lock();
        map.remove(&entity);
    }
    fn remove_entity_component(&self, entity: Entity, component_key: KString) {
        let mut map = self.map.lock();
        match map.entry(entity) {
            Entry::Occupied(mut entry) => {
                let comps = entry.get_mut();
                comps.retain(|e| {
                    let e = e.borrow_int_mut().unwrap();
                    e.component_key() != component_key
                });
                if comps.is_empty() {
                    entry.remove();
                }
            }
            Entry::Vacant(_) => {}
        }
    }
    fn get(&self, entity: Entity) -> Self::IntoIter {
        let map = self.map.lock();
        map.get(&entity).map(Clone::clone).unwrap_or_default()
    }

    fn len(&self) -> usize {
        self.map.lock().iter().map(|(_, v)| v.len()).sum()
    }
}
