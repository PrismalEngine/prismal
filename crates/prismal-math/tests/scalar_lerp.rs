use prismal_math::approx::assert_relative_eq;
use prismal_math::scalar::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_scalar_f32_lerp() {
    assert_relative_eq!(0.0f32.lerp(100.0, 0.7525), 75.25f32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_f64_lerp() {
    assert_relative_eq!(0.0.lerp(100.0, 0.7525), 75.25);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i8_lerp() {
    assert_eq!(0i8.lerp(100i8, 0.7525), 75i8);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i16_lerp() {
    assert_eq!(0i16.lerp(100i16, 0.7525), 75i16);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i32_lerp() {
    assert_eq!(0i32.lerp(100i32, 0.7525), 75i32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i64_lerp() {
    assert_eq!(0i64.lerp(100i64, 0.7525), 75i64);
}
