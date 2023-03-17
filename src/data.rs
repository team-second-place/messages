use crate::wasm_bindgen;
use crate::{Deserialize, Serialize};

// TODO: replace with real data like from the low level design

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraFrame {
    pub image: Vec<u8>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DutyCycle {
    pub duty_cycle: f64,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntruderAlert {
    pub image: Vec<u8>,
    pub timestamp: String,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightEffectDesiredBrightness {
    lumens: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightEffectForceEndBrightness {
    lumens: u32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LightEffect {
    LightEffectDesiredBrightness(LightEffectDesiredBrightness),
    LightEffectForceEndBrightness(LightEffectForceEndBrightness),
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    effect: LightEffect,
    weekday: u8,
    hour: u8,
    minute: u8,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    name: String,
    timers: Vec<Timer>,
}

// TODO: https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "variant", content = "data")]
pub enum Data {
    CameraFrame(CameraFrame),
    DutyCycle(DutyCycle),
    IntruderAlert(IntruderAlert),
    Preferences(Preferences),
}
