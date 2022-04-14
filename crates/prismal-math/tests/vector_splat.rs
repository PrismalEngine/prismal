use prismal_math::vector::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_vec2_splat() {
    let v = Vec2::splat(-1.5);
    assert_eq!(v.x, -1.5);
    assert_eq!(v.y, -1.5);
}

#[test]
#[wasm_bindgen_test]
fn test_dvec2_splat() {
    let v = DVec2::splat(-1.25);
    assert_eq!(v.x, -1.25);
    assert_eq!(v.y, -1.25);
}

#[test]
#[wasm_bindgen_test]
fn test_ivec2_splat() {
    let v = IVec2::splat(-2);
    assert_eq!(v.x, -2);
    assert_eq!(v.y, -2);
}

#[test]
#[wasm_bindgen_test]
fn test_uvec2_splat() {
    let v = UVec2::splat(2);
    assert_eq!(v.x, 2);
    assert_eq!(v.y, 2);
}

#[test]
#[wasm_bindgen_test]
fn test_vec3_splat() {
    let v = Vec3::splat(-1.5);
    assert_eq!(v.x, -1.5);
    assert_eq!(v.y, -1.5);
    assert_eq!(v.z, -1.5);
}

#[test]
#[wasm_bindgen_test]
fn test_dvec3_splat() {
    let v = DVec3::splat(-1.25);
    assert_eq!(v.x, -1.25);
    assert_eq!(v.y, -1.25);
    assert_eq!(v.z, -1.25);
}

#[test]
#[wasm_bindgen_test]
fn test_ivec3_splat() {
    let v = IVec3::splat(-2);
    assert_eq!(v.x, -2);
    assert_eq!(v.y, -2);
    assert_eq!(v.z, -2);
}

#[test]
#[wasm_bindgen_test]
fn test_uvec3_splat() {
    let v = UVec3::splat(2);
    assert_eq!(v.x, 2);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 2);
}

#[test]
#[wasm_bindgen_test]
fn test_vec4_splat() {
    let v = Vec4::splat(-1.5);
    assert_eq!(v.x, -1.5);
    assert_eq!(v.y, -1.5);
    assert_eq!(v.z, -1.5);
    assert_eq!(v.w, -1.5);
}

#[test]
#[wasm_bindgen_test]
fn test_dvec4_splat() {
    let v = DVec4::splat(-1.25);
    assert_eq!(v.x, -1.25);
    assert_eq!(v.y, -1.25);
    assert_eq!(v.z, -1.25);
    assert_eq!(v.w, -1.25);
}

#[test]
#[wasm_bindgen_test]
fn test_ivec4_splat() {
    let v = IVec4::splat(-2);
    assert_eq!(v.x, -2);
    assert_eq!(v.y, -2);
    assert_eq!(v.z, -2);
    assert_eq!(v.w, -2);
}

#[test]
#[wasm_bindgen_test]
fn test_uvec4_splat() {
    let v = UVec4::splat(2);
    assert_eq!(v.x, 2);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 2);
    assert_eq!(v.w, 2);
}
