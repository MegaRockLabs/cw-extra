use cosmwasm_std::Binary;
use cosmwasm_schema::{cw_serde, QueryResponses};


#[cw_serde]
pub struct ValidSignatureResponse {
    pub is_valid: bool
}

#[cw_serde]
pub struct ValidSignaturesResponse {
    pub are_valid: Vec<bool>
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw81QueryMsg {
    #[returns(ValidSignatureResponse)]
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<Binary>
    },

    #[returns(ValidSignaturesResponse)]
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,
        payload: Option<Binary>
    }
}