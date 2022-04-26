mod common;

use common::component_storage::*;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_ecs_component_storage_insert_single_hash() {
    let TestContext { storage, .. } =
        setup_test::<TestComponentSingleHash, <TestComponentSingleHash as Component>::Storage>();

    assert_eq!(storage.len(), 2);
}

#[test]
#[wasm_bindgen_test]
fn test_ecs_component_storage_get_single_hash() {
    let TestContext {
        storage,
        ent_0,
        ent_1,
        ent_2,
        ..
    } = setup_test::<TestComponentSingleHash, <TestComponentSingleHash as Component>::Storage>();

    let ent_0_comps = storage.get(ent_0);
    let ent_0_comps = ent_0_comps.into_iter();
    assert_eq!(ent_0_comps.clone().count(), 1);

    // Find by key for ent_0
    {
        let comp_0a = ent_0_comps
            .clone()
            .find(|x| x.component_key() == "ent_0:comp_0a");
        let comp_0b = ent_0_comps
            .clone()
            .find(|x| x.component_key() == "ent_0:comp_0b");
        let comp_1 = ent_0_comps
            .clone()
            .find(|x| x.component_key() == "ent_1:comp_1");
        assert!(comp_0a.is_none());
        assert!(comp_0b.is_some());
        assert!(comp_1.is_none());

        assert_eq!(comp_0b.unwrap().1, 21);
    }

    let ent_1_comps = storage.get(ent_1);
    let ent_1_comps = ent_1_comps.into_iter();
    assert_eq!(ent_1_comps.clone().count(), 1);

    // Find by key for ent_1
    {
        let comp_0a = ent_1_comps
            .clone()
            .find(|x| x.component_key() == "ent_0:comp_0a");
        let comp_0b = ent_1_comps
            .clone()
            .find(|x| x.component_key() == "ent_0:comp_0b");
        let comp_1 = ent_1_comps
            .clone()
            .find(|x| x.component_key() == "ent_1:comp_1");
        assert!(comp_0a.is_none());
        assert!(comp_0b.is_none());
        assert!(comp_1.is_some());

        assert_eq!(comp_1.unwrap().1, 7);
    }

    let ent_2_comps = storage.get(ent_2);
    let ent_2_comps = ent_2_comps.into_iter();
    assert_eq!(ent_2_comps.clone().count(), 0);
}

#[test]
#[wasm_bindgen_test]
fn test_ecs_component_storage_remove_entity_single_hash() {
    let TestContext {
        storage,
        ent_0,
        ent_1,
        ent_2,
        ..
    } = setup_test::<TestComponentSingleHash, <TestComponentSingleHash as Component>::Storage>();

    // Before any remove
    {
        let ent_0_comps = storage.get(ent_0);
        let ent_0_comps = ent_0_comps.into_iter();
        assert!(ent_0_comps.clone().next().is_some());

        let ent_1_comps = storage.get(ent_1);
        let ent_1_comps = ent_1_comps.into_iter();
        assert!(ent_1_comps.clone().next().is_some());

        let ent_2_comps = storage.get(ent_2);
        let ent_2_comps = ent_2_comps.into_iter();
        assert!(ent_2_comps.clone().next().is_none());
    }

    storage.remove_entity(ent_0);

    // After remove ent_0
    {
        let ent_0_comps = storage.get(ent_0);
        let ent_0_comps = ent_0_comps.into_iter();
        assert!(ent_0_comps.clone().next().is_none());

        let ent_1_comps = storage.get(ent_1);
        let ent_1_comps = ent_1_comps.into_iter();
        assert!(ent_1_comps.clone().next().is_some());

        let ent_2_comps = storage.get(ent_2);
        let ent_2_comps = ent_2_comps.into_iter();
        assert!(ent_2_comps.clone().next().is_none());
    }

    storage.remove_entity(ent_1);

    // After remove ent_1
    {
        let ent_0_comps = storage.get(ent_0);
        let ent_0_comps = ent_0_comps.into_iter();
        assert!(ent_0_comps.clone().next().is_none());

        let ent_1_comps = storage.get(ent_1);
        let ent_1_comps = ent_1_comps.into_iter();
        assert!(ent_1_comps.clone().next().is_none());

        let ent_2_comps = storage.get(ent_2);
        let ent_2_comps = ent_2_comps.into_iter();
        assert!(ent_2_comps.clone().next().is_none());
    }
}
