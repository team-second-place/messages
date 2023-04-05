use crate::wasm_bindgen;
use crate::{Deserialize, Serialize};

// TODO: replace with real commands per the low level design
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraFeedInterest {
    pub wants_camera_feed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "variant", content = "data")]
// TODO: https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
pub enum Command {
    CameraFeedInterest(CameraFeedInterest),
}
