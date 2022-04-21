use prismal_assets_core::type_id::*;

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_asset_type_id_serde() {
    use serde_test::{assert_tokens, Token};
    assert_tokens(&AssetTypeId::Group, &[Token::U16(0)]);
    assert_tokens(&AssetTypeId::Scene, &[Token::U16(1)]);
    assert_tokens(&AssetTypeId::Material, &[Token::U16(2)]);
    assert_tokens(&AssetTypeId::Texture, &[Token::U16(3)]);
    assert_tokens(&AssetTypeId::StaticMesh, &[Token::U16(4)]);
    assert_tokens(&AssetTypeId::SkeletalMesh, &[Token::U16(5)]);
    assert_tokens(&AssetTypeId::Other(12), &[Token::U16(12)]);
}
