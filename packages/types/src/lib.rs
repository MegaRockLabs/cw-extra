pub mod wasm;


#[cfg(feature = "registry_multi")]
pub use protos::registry_query_multi as registry_query;
#[cfg(not(feature = "registry_multi"))]
pub use protos::registry_query_one as registry_query;


#[cfg(feature = "account_multi")]
pub use {
    protos::valid_signature_multi as valid_signature_query,
    protos::account_query_multi as account_query,
};
#[cfg(not(feature = "account_multi"))]
pub use {
    protos::valid_signature_one as valid_signature_query,
    protos::account_query_one as account_query,
};

pub use protos::{
    registry_execute, 
    account_execute, 
};

use cosmwasm_schema::cw_serde;

// see cw1::CanExecuteResponse
// copy-paste to reduce number of dependencies
#[cw_serde]
pub struct CanExecuteResponse {
    pub can_execute: bool,
}

// used by both cw81 and cw82
#[cw_serde]
pub struct ValidSignatureResponse {
    pub is_valid: bool
}

// used by both cw81 and cw82 under `account_multi` feature
#[cw_serde]
pub struct ValidSignaturesResponse {
    pub are_valid: Vec<bool>
}

