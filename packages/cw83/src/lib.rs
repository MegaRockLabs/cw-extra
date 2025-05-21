mod msg;

pub const CREATE_ACCOUNT_REPLY_ID : u64 = 82;
pub const INTERFACE_NAME: &str = "crates:cw83";

pub use types::{registry_execute, registry_query};
pub use msg::*;
