use wasm_bindgen_test::*;

use prismal_utils::compare::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_serde() {
    use serde_test::{assert_tokens, Token};
    assert_tokens(
        &PartialCompareFunction(CompareFunction::LessEqual, 42.0f32),
        &[
            Token::TupleStruct {
                name: "PartialCompareFunction",
                len: 2,
            },
            Token::UnitVariant {
                name: "CompareFunction",
                variant: "less-equal",
            },
            Token::F32(42.0f32),
            Token::TupleStructEnd,
        ],
    );
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_lhs_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::Equal, 42.0f32);
    assert!(cmp0.eval_lhs(&42.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::Equal, "abc");
    assert!(cmp1.eval_lhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_lhs_not_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::NotEqual, 42.0f32);
    assert!(cmp0.eval_lhs(&21.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::NotEqual, "def");
    assert!(cmp1.eval_lhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_lhs_less() {
    let cmp0 = PartialCompareFunction(CompareFunction::Less, 42.0f32);
    assert!(cmp0.eval_lhs(&21.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::Less, "def");
    assert!(cmp1.eval_lhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_lhs_less_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::LessEqual, 42.0f32);
    assert!(cmp0.eval_lhs(&21.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::LessEqual, 21.0f32);
    assert!(cmp1.eval_lhs(&21.0f32));

    let cmp2 = PartialCompareFunction(CompareFunction::LessEqual, "def");
    assert!(cmp2.eval_lhs(&"abc"));

    let cmp3 = PartialCompareFunction(CompareFunction::LessEqual, "abc");
    assert!(cmp3.eval_lhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_lhs_greater() {
    let cmp0 = PartialCompareFunction(CompareFunction::Greater, 21.0f32);
    assert!(cmp0.eval_lhs(&42.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::Greater, "abc");
    assert!(cmp1.eval_lhs(&"def"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_lhs_greater_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::GreaterEqual, 21.0f32);
    assert!(cmp0.eval_lhs(&42.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::GreaterEqual, 21.0f32);
    assert!(cmp1.eval_lhs(&21.0f32));

    let cmp2 = PartialCompareFunction(CompareFunction::GreaterEqual, "abc");
    assert!(cmp2.eval_lhs(&"def"));

    let cmp3 = PartialCompareFunction(CompareFunction::GreaterEqual, "abc");
    assert!(cmp3.eval_lhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_rhs_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::Equal, 42.0f32);
    assert!(cmp0.eval_rhs(&42.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::Equal, "abc");
    assert!(cmp1.eval_rhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_rhs_not_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::NotEqual, 42.0f32);
    assert!(cmp0.eval_rhs(&21.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::NotEqual, "def");
    assert!(cmp1.eval_rhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_rhs_less() {
    let cmp0 = PartialCompareFunction(CompareFunction::Less, 21.0f32);
    assert!(cmp0.eval_rhs(&42.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::Less, "abc");
    assert!(cmp1.eval_rhs(&"def"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_rhs_less_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::LessEqual, 21.0f32);
    assert!(cmp0.eval_rhs(&42.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::LessEqual, 21.0f32);
    assert!(cmp1.eval_rhs(&21.0f32));

    let cmp2 = PartialCompareFunction(CompareFunction::LessEqual, "abc");
    assert!(cmp2.eval_rhs(&"def"));

    let cmp3 = PartialCompareFunction(CompareFunction::LessEqual, "abc");
    assert!(cmp3.eval_rhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_rhs_greater() {
    let cmp0 = PartialCompareFunction(CompareFunction::Greater, 42.0f32);
    assert!(cmp0.eval_rhs(&21.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::Greater, "def");
    assert!(cmp1.eval_rhs(&"abc"));
}

#[test]
#[wasm_bindgen_test]
fn test_partial_compare_function_eval_rhs_greater_equal() {
    let cmp0 = PartialCompareFunction(CompareFunction::GreaterEqual, 42.0f32);
    assert!(cmp0.eval_rhs(&21.0f32));

    let cmp1 = PartialCompareFunction(CompareFunction::GreaterEqual, 21.0f32);
    assert!(cmp1.eval_rhs(&21.0f32));

    let cmp2 = PartialCompareFunction(CompareFunction::GreaterEqual, "def");
    assert!(cmp2.eval_rhs(&"abc"));

    let cmp3 = PartialCompareFunction(CompareFunction::GreaterEqual, "abc");
    assert!(cmp3.eval_rhs(&"abc"));
}
