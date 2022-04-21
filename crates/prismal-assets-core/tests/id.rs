use prismal_assets_core::id::*;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_asset_id_serde() {
    use serde_test::{assert_tokens, Token};
    assert_tokens(&AssetId::from(42), &[Token::U64(42)]);
}

#[test]
#[wasm_bindgen_test]
fn test_asset_id_factory_case() {
    let a = asset_id("a/b/c");
    let b = asset_id("a/B/c");
    assert_eq!(a, b);
}

#[test]
#[wasm_bindgen_test]
fn test_asset_id_factory_separator() {
    let a = asset_id("a\\b/c");
    let b = asset_id("a/b\\c");
    let c = asset_id("a/b/c");
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}

#[test]
#[wasm_bindgen_test]
fn test_asset_id_factory_trim() {
    let a = asset_id("a/b/c ");
    let b = asset_id("  a/b/c");
    let c = asset_id("a/b/c");
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}
