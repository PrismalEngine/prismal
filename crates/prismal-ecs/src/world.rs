use std::any::TypeId;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

use prismal_utils::hash::int::IntHasherBuilder;
use prismal_utils::prelude::InteriorMut;
use prismal_utils::prelude::Rc;
use prismal_utils::sync::Mutex;

use crate::component::storage::component_storage::AnyComponentStorage;
use crate::component::storage::component_storage::ComponentStorage;
use crate::component::*;
use crate::entity::Entity;

pub struct World {
    live_entities: Mutex<HashSet<Entity, IntHasherBuilder>>,
    next_index: Mutex<u64>,
    storages: Mutex<HashMap<TypeId, Rc<dyn AnyComponentStorage>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            live_entities: Mutex::new(HashSet::with_hasher(IntHasherBuilder)),
            next_index: Mutex::new(1),
            storages: Mutex::new(HashMap::new()),
        }
    }

    pub fn entity_is_alive(&self, entity: Entity) -> bool {
        let live_entities = self.live_entities.borrow_int_mut().unwrap();
        live_entities.contains(&entity)
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

    pub fn register_component<C: Component + 'static>(&self) {
        let mut storages = self.storages.borrow_int_mut().unwrap();
        if let Entry::Vacant(entry) = storages.entry(TypeId::of::<C>()) {
            entry.insert(Rc::new(C::Storage::new()));
        }
    }

    pub fn storage<C: Component + 'static>(&self) -> Option<Rc<C::Storage>> {
        let storages = self.storages.borrow_int_mut().unwrap();
        storages
            .get(&TypeId::of::<C>())
            .and_then(|x| x.clone().downcast_arc::<C::Storage>().ok())
    }
}
