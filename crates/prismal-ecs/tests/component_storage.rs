use wasm_bindgen_test::*;

use prismal_ecs::component::storage::component_storage::ComponentStorage;
use prismal_ecs::component::storage::hash_map_component_storage::HashMapComponentStorage;
use prismal_ecs::component::storage::multi_hash_map_component_storage::MultiHashMapComponentStorage;
use prismal_ecs::component::*;
use prismal_ecs::entity::Entity;
use prismal_ecs::world::*;

use prismal_utils::shared::Rc;
use prismal_utils::string::key::KString;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, Default)]
struct TestComponentSingleHash(KString, i32);

impl ComponentKey for TestComponentSingleHash {
    fn key(&self) -> KString {
        self.0.clone()
    }
}
impl Component for TestComponentSingleHash {
    type Storage = HashMapComponentStorage<Self>;
}

#[derive(Debug, Clone, Default)]
struct TestComponentMultiHash(KString, i32);

impl ComponentKey for TestComponentMultiHash {
    fn key(&self) -> KString {
        self.0.clone()
    }
}
impl Component for TestComponentMultiHash {
    type Storage = MultiHashMapComponentStorage<Self>;
}

struct TestContext<S: ComponentStorage> {
    _world: World,
    ent_0: Entity,
    ent_1: Entity,
    storage: Rc<S>,
}

fn setup_test<C, S>() -> TestContext<S>
where
    S: ComponentStorage,
    C: Component<Storage = S> + 'static,
{
    let world = World::new();
    world.register_component::<C>();

    let ent_0 = world.spawn_entity();
    let ent_1 = world.spawn_entity();

    let storage = world.storage::<C>();
    assert!(storage.is_some());

    let storage = storage.unwrap();
    assert!(storage.is_empty());

    TestContext {
        _world: world,
        ent_0,
        ent_1,
        storage,
    }
}

#[test]
#[wasm_bindgen_test]
fn test_ecs_component_storage_insert_single_hash() {
    let context =
        setup_test::<TestComponentSingleHash, <TestComponentSingleHash as Component>::Storage>();

    let ent_0 = context.ent_0;
    let ent_1 = context.ent_1;
    let storage = context.storage;

    let comp_0a = TestComponentSingleHash(KString::from_ref("comp_0a"), 42);
    let comp_0b = TestComponentSingleHash(KString::from_ref("comp_0b"), 21);
    let comp_1 = TestComponentSingleHash(KString::from_ref("comp_1"), 21);

    storage.insert(ent_0, comp_0a);
    storage.insert(ent_0, comp_0b);
    storage.insert(ent_1, comp_1);

    assert_eq!(storage.len(), 2);
}

#[test]
#[wasm_bindgen_test]
fn test_ecs_component_storage_insert_multi_hash() {
    let context =
        setup_test::<TestComponentMultiHash, <TestComponentMultiHash as Component>::Storage>();

    let ent_0 = context.ent_0;
    let ent_1 = context.ent_1;
    let storage = context.storage;

    let comp_0a = TestComponentMultiHash(KString::from_ref("comp_0a"), 42);
    let comp_0b = TestComponentMultiHash(KString::from_ref("comp_0b"), 21);
    let comp_1 = TestComponentMultiHash(KString::from_ref("comp_1"), 21);

    storage.insert(ent_0, comp_0a);
    storage.insert(ent_0, comp_0b);
    storage.insert(ent_1, comp_1);

    assert_eq!(storage.len(), 3);
}
