mod msg;

pub const INTERFACE_NAME: &str = "crates:cw84";
#[cfg(feature = "multi")]
pub const INTERFACE_NAME_MULTI: &str = "crates:cw84-multi";

pub use types::{signed_execute, signed_query};
pub use types::wasm::Binary;
pub use msg::*;
