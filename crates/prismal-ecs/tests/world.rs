use wasm_bindgen_test::*;

use prismal_ecs::component::storage::component_storage::ComponentStorage;
use prismal_ecs::component::storage::hash_map_component_storage::HashMapComponentStorage;
use prismal_ecs::component::*;
use prismal_ecs::world::*;

use prismal_utils::string::key::KString;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, Default)]
struct TestComponentA(KString, i32);

impl Component for TestComponentA {
    type Storage = HashMapComponentStorage<Self>;
    fn key(&self) -> KString {
        self.0.clone()
    }
}

#[test]
#[wasm_bindgen_test]
fn test_ecs_world_register_component() {
    let world = World::new();
    world.register_component::<TestComponentA>();

    let storage = world.storage::<TestComponentA>();
    assert!(storage.is_some());

    let storage = storage.unwrap();
    assert!(storage.is_empty());

    let ent_1 = world.spawn_entity();
    let ent_2 = world.spawn_entity();

    let comp_a = TestComponentA(KString::from_ref("comp_a"), 42);
    storage.insert(ent_1, comp_a);

    let comp_b = TestComponentA(KString::from_ref("comp_b"), 21);
    storage.insert(ent_2, comp_b);

    assert_eq!(storage.len(), 2);
}
