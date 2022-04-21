use prismal_assets_core::key::*;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_asset_key_serde() {
    use serde_test::{assert_tokens, Token};
    assert_tokens(&AssetKey::from(42), &[Token::U64(42)]);
}

#[test]
#[wasm_bindgen_test]
fn test_asset_key_factory_case() {
    let a = "a/b/c".asset_key();
    let b = "a/B/c".asset_key();
    assert_eq!(a, b);
}

#[test]
#[wasm_bindgen_test]
fn test_asset_key_factory_separator_char() {
    let a = "a\\b/c".asset_key();
    let b = "a/b\\c".asset_key();
    let c = "a/b/c".asset_key();
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}

#[test]
#[wasm_bindgen_test]
fn test_asset_key_factory_separator_repeat() {
    let a = "a\\\\b/c".asset_key();
    let b = "a//b\\c".asset_key();
    let c = "a/b/c".asset_key();
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}

#[test]
#[wasm_bindgen_test]
fn test_asset_key_factory_trim() {
    let a = "a/b/c ".asset_key();
    let b = "  a/b/c".asset_key();
    let c = "a/b/c".asset_key();
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}
