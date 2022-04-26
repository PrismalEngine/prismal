use wasm_bindgen_test::*;

use prismal_ecs::component::storage::component_storage::ComponentStorage;
use prismal_ecs::component::storage::hash_map_component_storage::HashMapComponentStorage;
use prismal_ecs::component::storage::multi_hash_map_component_storage::MultiHashMapComponentStorage;
use prismal_ecs::component::*;
use prismal_ecs::world::*;

use prismal_utils::string::key::KString;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, Default)]
struct TestComponentA(KString, i32);

impl ComponentKey for TestComponentA {
    fn key(&self) -> KString {
        self.0.clone()
    }
}
impl Component for TestComponentA {
    type Storage = HashMapComponentStorage<Self>;
}

#[derive(Debug, Clone, Default)]
struct TestComponentB(KString, f32);

impl ComponentKey for TestComponentB {
    fn key(&self) -> KString {
        self.0.clone()
    }
}
impl Component for TestComponentB {
    type Storage = MultiHashMapComponentStorage<Self>;
}

#[test]
#[wasm_bindgen_test]
fn test_ecs_world_register_component() {
    let world = World::new();
    world.register_component::<TestComponentA>();
    world.register_component::<TestComponentB>();

    let storage_a = world.storage::<TestComponentA>();
    let storage_b = world.storage::<TestComponentB>();
    assert!(storage_a.is_some());
    assert!(storage_b.is_some());

    let storage_a = storage_a.unwrap();
    let storage_b = storage_b.unwrap();
    assert!(storage_a.is_empty());
    assert!(storage_b.is_empty());
}
