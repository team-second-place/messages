use crate::wasm_bindgen;
use crate::{Deserialize, Serialize};

// TODO: replace with real data like from the low level design

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentLight {
    pub on: bool,
}

// TODO: https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "variant", content = "data")]
pub enum Data {
    CurrentLight(CurrentLight),
}
