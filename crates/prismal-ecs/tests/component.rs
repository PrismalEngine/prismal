use wasm_bindgen_test::*;

use prismal_ecs::component::storage::component_storage::ComponentStorage;
use prismal_ecs::component::storage::hash_map_component_storage::HashMapComponentStorage;
use prismal_ecs::component::*;
use prismal_ecs::entity::Entity;

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
fn test_component_trait_object() {
    let entity = Entity::from_index(1);

    let mut storage = <TestComponentA as Component>::Storage::new();
    storage.insert(
        entity,
        TestComponentA(KString::from_ref("key_test_comp_a"), 42),
    );

    let comp: &dyn AnyComponent = storage.get_ref(entity).unwrap();
    assert_eq!(comp.key(), "key_test_comp_a");
}
