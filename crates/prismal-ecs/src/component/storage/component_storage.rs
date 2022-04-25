use downcast::AnySync;

use crate::entity::Entity;

pub trait ComponentStorage: Send + Sync {
    type Stored: Send + Sync + Clone;
    type IntoIter: IntoIterator<Item = Self::Stored>;

    fn new() -> Self;
    fn insert(&self, entity: Entity, component: Self::Stored);
    fn remove_entity(&self, entity: Entity);
    fn get(&self, entity: Entity) -> Self::IntoIter;

    #[must_use]
    fn len(&self) -> usize;

    #[must_use]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait AnyComponentStorage: AnySync {}
impl<T: AnySync, C: ComponentStorage<Stored = T> + 'static> AnyComponentStorage for C {}

downcast::downcast_sync!(dyn AnyComponentStorage);
