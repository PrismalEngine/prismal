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

impl Component for TestComponentA {
    type Storage = HashMapComponentStorage<Self>;
    fn key(&self) -> KString {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Default)]
struct TestComponentB(KString, f32);

impl Component for TestComponentB {
    type Storage = MultiHashMapComponentStorage<Self>;
    fn key(&self) -> KString {
        self.0.clone()
    }
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

    let ent_1 = world.spawn_entity();
    let ent_2 = world.spawn_entity();

    let comp_a0 = TestComponentA(KString::from_ref("ent_1:comp_a0"), 42);
    let comp_a1 = TestComponentA(KString::from_ref("ent_2:comp_a1"), 21);
    let comp_a2 = TestComponentA(KString::from_ref("ent_2:comp_a2"), 12);

    storage_a.insert(ent_1, comp_a0);
    storage_a.insert(ent_2, comp_a1);
    storage_a.insert(ent_2, comp_a2);
    assert_eq!(storage_a.len(), 2);

    let comp_b0 = TestComponentB(KString::from_ref("ent_1:comp_b0"), 1.5);
    let comp_b1 = TestComponentB(KString::from_ref("ent_1:comp_b1"), -0.5);

    storage_b.insert(ent_1, comp_b0);
    storage_b.insert(ent_1, comp_b1);
    assert_eq!(storage_b.len(), 2);
}
