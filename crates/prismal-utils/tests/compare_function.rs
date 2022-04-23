use wasm_bindgen_test::*;

use prismal_utils::compare::CompareFunction;

wasm_bindgen_test_configure!(run_in_browser);

macro_rules! gen_test_compare_function_serde {
    ($variant:ident, $ser_name:expr) => {{
        use serde_test::{assert_tokens, Token};
        assert_tokens(
            &CompareFunction::$variant,
            &[Token::UnitVariant {
                name: "CompareFunction",
                variant: $ser_name,
            }],
        );
    }};
}

#[test]
#[wasm_bindgen_test]
fn test_compare_function_serde() {
    gen_test_compare_function_serde!(Equal, "equal");
    gen_test_compare_function_serde!(NotEqual, "not-equal");
    gen_test_compare_function_serde!(Less, "less");
    gen_test_compare_function_serde!(LessEqual, "less-equal");
    gen_test_compare_function_serde!(Greater, "greater");
    gen_test_compare_function_serde!(GreaterEqual, "greater-equal");
}

#[test]
#[wasm_bindgen_test]
fn test_compare_function_eval_equal() {
    let cmp = CompareFunction::Equal;
    assert!(cmp.eval(&42, &42));

    let s0 = "abc";
    let s1 = "abc";
    assert!(cmp.eval(s0, s1));
}

#[test]
#[wasm_bindgen_test]
fn test_compare_function_eval_not_equal() {
    let cmp = CompareFunction::NotEqual;
    assert!(cmp.eval(&21, &42));

    let s0 = "abc";
    let s1 = "def";
    assert!(cmp.eval(s0, s1));
}

#[test]
#[wasm_bindgen_test]
fn test_compare_function_eval_less() {
    let cmp = CompareFunction::Less;
    assert!(cmp.eval(&21, &42));

    let s0 = "abc";
    let s1 = "def";
    assert!(cmp.eval(s0, s1));
}

#[test]
#[wasm_bindgen_test]
fn test_compare_function_eval_less_equal() {
    let cmp = CompareFunction::LessEqual;
    assert!(cmp.eval(&21, &42));
    assert!(cmp.eval(&42, &42));

    let s0 = "abc";
    let s1 = "def";
    let s2 = "def";
    assert!(cmp.eval(s0, s1));
    assert!(cmp.eval(s1, s2));
}

#[test]
#[wasm_bindgen_test]
fn test_compare_function_eval_greater() {
    let cmp = CompareFunction::Greater;
    assert!(cmp.eval(&42, &21));

    let s0 = "def";
    let s1 = "abc";
    assert!(cmp.eval(s0, s1));
}

#[test]
#[wasm_bindgen_test]
fn test_compare_function_eval_greater_equal() {
    let cmp = CompareFunction::GreaterEqual;
    assert!(cmp.eval(&42, &21));
    assert!(cmp.eval(&42, &42));

    let s0 = "def";
    let s1 = "abc";
    let s2 = "abc";
    assert!(cmp.eval(s0, s1));
    assert!(cmp.eval(s1, s2));
}
