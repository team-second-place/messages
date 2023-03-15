//! Test suite for Node
#![cfg(target_arch = "wasm32")]

// use pretty_assertions::assert_eq;

use messages::*;
use pretty_assertions::assert_str_eq;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn create_command() {
    setup();

    let input = FromUser::Command(Command::ChangeLight(ChangeLight { to: true }));
    let input_as_js_value = serde_wasm_bindgen::to_value(&input).unwrap();

    let encoded = encode_from_user(input_as_js_value.clone()).unwrap();
    tracing::info!("{encoded:?}");

    let output_as_js_value = decode_from_user(&encoded).unwrap();

    assert_str_eq!(
        format!("{:#?}", input_as_js_value),
        format!("{:#?}", output_as_js_value)
    );
}
