use cfg_if::cfg_if;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use noop_proc_macro::wasm_bindgen;

cfg_if! {
    if #[cfg(any(feature = "serde", target_arch = "wasm32"))] {
        use serde::{Deserialize, Serialize};
    } else {
        use noop_proc_macro::{Deserialize, Serialize};
    }
}

cfg_if! {
    if #[cfg(feature = "tracing")] {
        pub fn setup_tracing() {
            #[cfg(target_arch = "wasm32")]
            tracing_wasm::set_as_global_default();
        }
    } else {
        pub fn setup_tracing() {}
    }
}

/// Set up this WASM library
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn setup() {
    setup_tracing();
}

pub mod command;
pub use command::*;
pub mod communication;
pub use communication::*;
pub mod data;
pub use data::*;

#[cfg(feature = "encoding")]
pub mod encoding;
#[cfg(feature = "encoding")]
pub use encoding::*;

pub type MicrocontrollerId = String;
pub type UserId = String;

pub type UserName = String;
pub type UserPassword = String;
