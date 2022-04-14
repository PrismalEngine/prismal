use prismal_math::vector::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_vec2_from_tuple() {
    let v = Vec2::from((-1.5, 2.5));
    assert_eq!(v.x, -1.5);
    assert_eq!(v.y, 2.5);
}

#[test]
#[wasm_bindgen_test]
fn test_dvec2_from_tuple() {
    let v = DVec2::from((-1.25, 2.25));
    assert_eq!(v.x, -1.25);
    assert_eq!(v.y, 2.25);
}

#[test]
#[wasm_bindgen_test]
fn test_ivec2_from_tuple() {
    let v = IVec2::from((-1, 2));
    assert_eq!(v.x, -1);
    assert_eq!(v.y, 2);
}

#[test]
#[wasm_bindgen_test]
fn test_uvec2_from_tuple() {
    let v = UVec2::from((1, 2));
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 2);
}

#[test]
#[wasm_bindgen_test]
fn test_vec3_from_tuple() {
    let v = Vec3::from((-1.5, 2.5, 3.5));
    assert_eq!(v.x, -1.5);
    assert_eq!(v.y, 2.5);
    assert_eq!(v.z, 3.5);
}

#[test]
#[wasm_bindgen_test]
fn test_dvec3_from_tuple() {
    let v = DVec3::from((-1.25, 2.25, 3.25));
    assert_eq!(v.x, -1.25);
    assert_eq!(v.y, 2.25);
    assert_eq!(v.z, 3.25);
}

#[test]
#[wasm_bindgen_test]
fn test_ivec3_from_tuple() {
    let v = IVec3::from((-1, 2, 3));
    assert_eq!(v.x, -1);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 3);
}

#[test]
#[wasm_bindgen_test]
fn test_uvec3_from_tuple() {
    let v = UVec3::from((1, 2, 3));
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 3);
}

#[test]
#[wasm_bindgen_test]
fn test_vec4_from_tuple() {
    let v = Vec4::from((-1.5, 2.5, 3.5, 4.5));
    assert_eq!(v.x, -1.5);
    assert_eq!(v.y, 2.5);
    assert_eq!(v.z, 3.5);
    assert_eq!(v.w, 4.5);
}

#[test]
#[wasm_bindgen_test]
fn test_dvec4_from_tuple() {
    let v = DVec4::from((-1.25, 2.25, 3.25, 4.25));
    assert_eq!(v.x, -1.25);
    assert_eq!(v.y, 2.25);
    assert_eq!(v.z, 3.25);
    assert_eq!(v.w, 4.25);
}

#[test]
#[wasm_bindgen_test]
fn test_ivec4_from_tuple() {
    let v = IVec4::from((-1, 2, 3, 4));
    assert_eq!(v.x, -1);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 3);
    assert_eq!(v.w, 4);
}

#[test]
#[wasm_bindgen_test]
fn test_uvec4_from_tuple() {
    let v = UVec4::from((1, 2, 3, 4));
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 2);
    assert_eq!(v.z, 3);
    assert_eq!(v.w, 4);
}
