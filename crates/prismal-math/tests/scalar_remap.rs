use prismal_math::approx::assert_relative_eq;
use prismal_math::scalar::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_scalar_f32_remap() {
    assert_relative_eq!(0.5f32.remap(0.0, 1.0, 0.0, 100.0), 50.0f32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_f64_remap() {
    assert_relative_eq!(0.5f64.remap(0.0, 1.0, 0.0, 100.0), 50.0);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i8_remap() {
    assert_eq!(5i8.remap(0, 10, 0, 100), 50);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i16_remap() {
    assert_eq!(5i16.remap(0, 10, 0, 100), 50);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i32_remap() {
    assert_eq!(5i32.remap(0, 10, 0, 100), 50);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i64_remap() {
    assert_eq!(5i64.remap(0, 10, 0, 100), 50);
}
