use prismal_math::approx::assert_relative_eq;
use prismal_math::scalar::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_scalar_f32_lerp_factor() {
    assert_relative_eq!(0.0f32.lerp_factor(100.0f32, 75.25f32), 0.7525f32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_f64_lerp_factor() {
    assert_relative_eq!(0.0f64.lerp_factor(100.0f64, 75.25f64), 0.7525f32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i8_lerp_factor() {
    assert_relative_eq!(0i8.lerp_factor(100i8, 75i8), 0.75f32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i16_lerp_factor() {
    assert_relative_eq!(0i16.lerp_factor(100i16, 75i16), 0.75f32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i32_lerp_factor() {
    assert_relative_eq!(0i32.lerp_factor(100i32, 75i32), 0.75f32);
}

#[test]
#[wasm_bindgen_test]
fn test_scalar_i64_lerp_factor() {
    assert_relative_eq!(0i64.lerp_factor(100i64, 75i64), 0.75f32);
}
