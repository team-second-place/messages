use crate::{command, data, MicrocontrollerId, UserId, UserName, UserPassword};
use crate::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: UserName,
    pub password: UserPassword,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Authenticate {
    pub microcontroller_id: MicrocontrollerId,
    pub login_info: LoginInfo,
}

// TODO: https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "variant", content = "data")]
pub enum FromUser {
    Command(command::Command),
    Authenticate(Authenticate),
}

// TODO: https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "variant", content = "data")]
pub enum ToMicrocontroller {
    Command(command::Command, UserId),
    LoginRequest(LoginInfo, UserId),
    UsersAreOnline,
    UsersAreOffline,
    /// Only exists to help me debug
    UsageError, // TODO: define as an enum with variants like Unauthenticated?
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Register {
    pub microcontroller_id: MicrocontrollerId,
}

// TODO: https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "variant", content = "data")]
pub enum FromMicrocontroller {
    Register(Register),
    BroadcastData(data::Data),
    UserSpecificData(data::Data, Vec<UserId>),
    LoginResponse(bool, UserId),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Authenticated(pub bool);

// TODO: https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/typescript_custom_section.html
#[derive(Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "variant", content = "data")]
pub enum ToUser {
    Data(data::Data),
    Authenticated(Authenticated),
    MicrocontrollerIsOffline,
    /// Only exists to help me debug
    UsageError,
}
