#![cfg(feature = "encoding")]

use paste::paste;

use crate::wasm_bindgen;

pub use rmp_serde::decode::Error as DecodeError;
pub use rmp_serde::encode::Error as EncodeError;

macro_rules! create_encoder_for {
    ($type: ty, $identifier: ident) => {
        paste! {
            pub fn [<encode_ $identifier _to_bytes>](
                structure: &$type,
            ) -> Result<Vec<u8>, EncodeError> {
                rmp_serde::to_vec(structure)
            }

            cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    #[wasm_bindgen]
                    pub fn [<encode_ $identifier>](
                        js_value: wasm_bindgen::JsValue,
                    ) -> Result<Box<[u8]>, wasm_bindgen::JsValue> {
                        let structure: $type = serde_wasm_bindgen::from_value(js_value)?;

                        [<encode_ $identifier _to_bytes>](&structure)
                            .map(Box::from)
                            .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
                    }
                } else {
                    pub use self::[<encode_ $identifier _to_bytes>] as [<encode_ $identifier>];
                }
            }
        }
    };
}

macro_rules! create_decoder_for {
    ($type: ty, $identifier: ident) => {
        paste! {
            pub fn [<decode_ $identifier _to_struct>](bytes: &[u8]) -> Result<$type, DecodeError> {
                    rmp_serde::from_slice(bytes)
            }

            cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    #[wasm_bindgen]
                    pub fn [<decode_ $identifier>](bytes: &[u8]) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
                        let structure = [<decode_ $identifier _to_struct>](bytes).map_err(|e| e.to_string())?;

                        serde_wasm_bindgen::to_value(&structure).map_err(|e| e.into())
                    }
                } else {
                    pub use self::[<decode_ $identifier _to_struct>] as [<decode_ $identifier>];
                }
            }
        }
    };
}

macro_rules! create_encoders_and_decoders_for {
    (
        $(
            ($type: ty, $identifier: ident)
        ),+

        $(,)?
    ) => {
        $(create_encoder_for!($type, $identifier);)+
        $(create_decoder_for!($type, $identifier);)+
    }
}

create_encoders_and_decoders_for!(
    (
        crate::communication::FromMicrocontroller,
        from_microcontroller
    ),
    (crate::communication::FromUser, from_user),
    (crate::communication::ToMicrocontroller, to_microcontroller),
    (crate::communication::ToUser, to_user),
);

#[cfg(test)]
mod tests {
    use crate::{decode_to_microcontroller, encode_to_microcontroller, ToMicrocontroller};

    #[test]
    fn can_encode_from_and_decode_to_same_value() {
        let to_microcontroller = ToMicrocontroller::UsersAreOffline;
        let to_microcontroller_encoded = encode_to_microcontroller(&to_microcontroller).unwrap();
        let message_decoded = decode_to_microcontroller(&to_microcontroller_encoded).unwrap();

        assert_eq!(to_microcontroller, message_decoded);
    }
}
