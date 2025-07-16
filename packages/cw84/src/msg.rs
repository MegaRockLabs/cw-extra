
use cosmwasm_schema::{cw_serde, QueryResponses};
use types::wasm::{Binary, CosmosMsg, Empty, Uint64};

#[cfg(feature = "multi")]
pub use types::{CanExecuteSignedResponse, ValidSignaturesResponse};
pub use types::{CanExecuteResponse, ValidSignatureResponse};


type SignedData = Binary;

/// Absolute minimum of a message that follows cw84 standard.
///  Same as cw1::Cw1ExecuteMsg
#[cw_serde]
pub enum Cw84ExecuteMsg<T = Empty>
where
    T: Clone + std::fmt::Debug + PartialEq + cosmwasm_schema::schemars::JsonSchema,
{
    /// Execute requests the contract to re-dispatch all these messages with the
    /// contract's address as sender. Every implementation has it's own logic to
    /// determine in
    Execute { 
        msgs: Vec<CosmosMsg<T>>,
        signed: Option<SignedData>,
    },

    /// ExecuteSigned requests the contract to use a custom signature verification scheme
    /// and after successful check, execute custom messages defined by the contract.
    #[cfg(feature = "multi")]
    ExecuteSigned { 
        msgs: Vec<Cw84ExecuteMsg<T>>, 
        signed: SignedData,
        nonce: Option<Uint64>,
    },

    #[cfg(not(feature = "multi"))]
    ExecuteSigned { 
        msg: Box<Cw84ExecuteMsg<T>>, 
        signed: SignedData,
        nonce: Option<Uint64>,
    }
}



/// Absolute minimum of a query that follows cw84 standard
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw84QueryMsg<T = Empty> 
where
    T: Clone + std::fmt::Debug + PartialEq + cosmwasm_schema::schemars::JsonSchema,
{

    #[returns(CanExecuteResponse)]
    CanExecute {
        sender      :   String,
        msg         :   CosmosMsg<T>,
    },

    #[cfg(not(feature = "multi"))]
    #[returns(CanExecuteResponse)]
    CanExecuteSigned {
        msg         :   Cw84ExecuteMsg<T>,
        signed      :   SignedData,
        nonce       :   Option<Uint64>,
    },
    
    #[cfg(feature = "multi")]
    #[returns(CanExecuteSignedResponse)]
    CanExecuteSigned {
        msgs        :   Vec<Cw84ExecuteMsg<T>>,
        signed      :   SignedData,
        nonce       :   Option<Uint64>,
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
        payload     :   Option<Vec<Binary>>
    }
}