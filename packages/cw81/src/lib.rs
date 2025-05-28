use cosmwasm_schema::{cw_serde, QueryResponses};
use types::wasm::Binary;


pub const INTERFACE_NAME: &str = "crates:cw81";


/// Absolute minimum of a query that follows cw82 standard
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw81QueryMsg {

    #[returns(ValidSignatureResponse)]
    ValidSignature {
        data        :   Binary,
        signature   :   Binary,
        payload     :   Option<Binary>
    },

    #[cfg(feature = "multi")]
    #[returns(ValidSignaturesResponse)]
    ValidSignatures {
        data        :   Vec<Binary>,
        signatures  :   Vec<Binary>,
        payload     :   Option<Binary>
    }

}

// No execute message is defined for cw81

pub use types::{valid_signature_query, ValidSignatureResponse};
#[cfg(feature = "multi")]
pub use types::ValidSignaturesResponse;
