pub use prismal_ecs::component::storage::component_storage::ComponentStorage;
pub use prismal_ecs::component::storage::hash_map_component_storage::HashMapComponentStorage;
pub use prismal_ecs::component::storage::multi_hash_map_component_storage::MultiHashMapComponentStorage;
pub use prismal_ecs::component::*;
pub use prismal_ecs::entity::Entity;
pub use prismal_ecs::world::*;

pub use prismal_utils::shared::Rc;
pub use prismal_utils::string::key::KString;

#[derive(Debug, Clone, Default)]
pub struct TestComponentSingleHash(KString, pub i32);

impl ComponentKey for TestComponentSingleHash {
    fn component_key(&self) -> KString {
        self.0.clone()
    }
}
impl Component for TestComponentSingleHash {
    type Storage = HashMapComponentStorage<Self>;
}

#[derive(Debug, Clone, Default)]
pub struct TestComponentMultiHash(KString, pub i32);

impl ComponentKey for TestComponentMultiHash {
    fn component_key(&self) -> KString {
        self.0.clone()
    }
}
impl Component for TestComponentMultiHash {
    type Storage = MultiHashMapComponentStorage<Self>;
}

impl From<(KString, i32)> for TestComponentSingleHash {
    fn from((k, v): (KString, i32)) -> Self {
        Self(k, v)
    }
}
impl From<(KString, i32)> for TestComponentMultiHash {
    fn from((k, v): (KString, i32)) -> Self {
        Self(k, v)
    }
}

#[allow(dead_code)]
pub struct TestContext<C, S>
where
    S: ComponentStorage,
    C: Component<Storage = S> + 'static,
{
    pub world: World,
    pub storage: Rc<S>,
    pub ent_0: Entity,
    pub ent_1: Entity,
    pub ent_2: Entity,
    pub comp_0a: C,
    pub comp_0b: C,
    pub comp_1: C,
}

pub fn setup_test<C, S>() -> TestContext<C, S>
where
    S: ComponentStorage<Stored = C>,
    C: From<(KString, i32)> + Component<Storage = S> + Clone + 'static,
{
    let world = World::new();
    world.register_component::<C>();

    let ent_0 = world.spawn_entity();
    let ent_1 = world.spawn_entity();
    let ent_2 = world.spawn_entity();

    let storage = world.storage::<C>();
    assert!(storage.is_some());

    let storage = storage.unwrap();
    assert!(storage.is_empty());

    let comp_0a = C::from((KString::from_ref("ent_0:comp_0a"), 42));
    let comp_0b = C::from((KString::from_ref("ent_0:comp_0b"), 21));
    let comp_1 = C::from((KString::from_ref("ent_1:comp_1"), 7));

    storage.insert(ent_0, comp_0a.clone());
    storage.insert(ent_0, comp_0b.clone());
    storage.insert(ent_1, comp_1.clone());

    TestContext {
        world,
        storage,
        ent_0,
        ent_1,
        ent_2,
        comp_0a,
        comp_0b,
        comp_1,
    }
}
