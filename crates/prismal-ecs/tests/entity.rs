use prismal_ecs::entity::Entity;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_ecs_entity_serde() {
    macro_rules! generate_test_entity_serde {
        ($index:expr) => {{
            use serde_test::{assert_tokens, Token};
            assert_tokens(&Entity::from_index($index), &[Token::U64($index)]);
            assert_tokens(&Entity::from_index($index * 10), &[Token::U64($index * 10)]);
            assert_tokens(
                &Entity::from_index($index * 100),
                &[Token::U64($index * 100)],
            );
            assert_tokens(
                &Entity::from_index($index * 1_000),
                &[Token::U64($index * 1_000)],
            );
            assert_tokens(
                &Entity::from_index($index * 10_000),
                &[Token::U64($index * 10_000)],
            );
        }};
    }
    generate_test_entity_serde!(0);
    generate_test_entity_serde!(1);
    generate_test_entity_serde!(2);
    generate_test_entity_serde!(3);
    generate_test_entity_serde!(4);
    generate_test_entity_serde!(5);
    generate_test_entity_serde!(6);
    generate_test_entity_serde!(7);
    generate_test_entity_serde!(8);
    generate_test_entity_serde!(9);
}

#[test]
#[wasm_bindgen_test]
fn test_ecs_entity_index() {
    macro_rules! generate_test_entity_index {
        ($index:expr) => {{
            assert_eq!(Entity::from_index($index).index(), $index);
            assert_eq!(Entity::from_index($index * 10).index(), $index * 10);
            assert_eq!(Entity::from_index($index * 100).index(), $index * 100,);
            assert_eq!(Entity::from_index($index * 1_000).index(), $index * 1_000,);
            assert_eq!(Entity::from_index($index * 10_000).index(), $index * 10_000,);
        }};
    }
    generate_test_entity_index!(0);
    generate_test_entity_index!(1);
    generate_test_entity_index!(2);
    generate_test_entity_index!(3);
    generate_test_entity_index!(4);
    generate_test_entity_index!(5);
    generate_test_entity_index!(6);
    generate_test_entity_index!(7);
    generate_test_entity_index!(8);
    generate_test_entity_index!(9);
}
