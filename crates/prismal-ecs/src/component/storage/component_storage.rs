use crate::entity::Entity;

pub trait ComponentStorage<T> {
    fn new() -> Self;
    fn insert(&mut self, entity: Entity, component: T);
    fn remove(&mut self, entity: Entity);
    fn get_ref(&self, entity: Entity) -> Option<&T>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut T>;
}
