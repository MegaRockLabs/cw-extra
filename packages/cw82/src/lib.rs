use cosmwasm_schema::{cw_serde, QueryResponses};
use types::wasm::{Binary, CosmosMsg, Empty};


pub const INTERFACE_NAME : &str = "crates.io:cw82";


/// Absolute minimum of a query that follows cw82 standard
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw82QueryMsg<T = Empty> {

    #[returns(CanExecuteResponse)]
    CanExecute {
        sender      :   String,
        msg         :   CosmosMsg<T>,
    },

    #[returns(ValidSignatureResponse)]
    ValidSignature {
        data        :   Binary,
        signatures  :   Binary,
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


/// Absolute minimum of a message that follows cw82 standard. Same as cw1::Cw1ExecuteMsg
#[cw_serde]
pub enum Cw82ExecuteMsg<T = Empty>
where
    T: Clone + std::fmt::Debug + PartialEq + cosmwasm_schema::schemars::JsonSchema,
{
    /// Execute requests the contract to re-dispatch all these messages with the
    /// contract's address as sender. Every implementation has it's own logic to
    /// determine in
    Execute { msgs: Vec<CosmosMsg<T>> },
}


pub use types::{account_query, account_execute, ValidSignatureResponse, CanExecuteResponse};
#[cfg(feature = "multi")]
pub use types::ValidSignaturesResponse;