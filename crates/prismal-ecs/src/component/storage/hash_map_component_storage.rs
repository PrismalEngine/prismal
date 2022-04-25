use std::collections::HashMap;

use prismal_utils::hash::int::IntHasherBuilder;

use crate::component::storage::component_storage::ComponentStorage;
use crate::entity::Entity;

pub struct HashMapComponentStorage<T> {
    map: HashMap<Entity, T, IntHasherBuilder>,
}

impl<T> ComponentStorage<T> for HashMapComponentStorage<T> {
    fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
    fn insert(&mut self, entity: Entity, component: T) {
        self.map.insert(entity, component);
    }
    fn remove(&mut self, entity: Entity) {
        self.map.remove(&entity);
    }
    fn get_ref(&self, entity: Entity) -> Option<&T> {
        self.map.get(&entity)
    }
    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.map.get_mut(&entity)
    }
}
