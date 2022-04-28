use prismal_ecs::component::storage::component_storage::ComponentStorage;
use prismal_ecs::component::storage::hash_map_component_storage::HashMapComponentStorage;
use prismal_ecs::component::*;
use prismal_ecs::entity::Entity;

use prismal_utils::shared::RcMut;
use prismal_utils::string::key::KString;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, Default)]
struct TestComponentA(KString, i32);

impl ComponentKey for TestComponentA {
    fn component_key(&self) -> KString {
        self.0.clone()
    }
}
impl Component for TestComponentA {
    type Storage = HashMapComponentStorage<Self>;
}

#[test]
#[wasm_bindgen_test]
fn test_component_trait_object() {
    let entity = Entity::from_index(1);

    let storage = <TestComponentA as Component>::Storage::new();
    storage.insert(
        entity,
        TestComponentA(KString::from_ref("key_test_comp_a"), 42),
    );

    let comp_arc = storage.get(entity).unwrap();
}
