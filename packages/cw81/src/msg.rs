use cosmwasm_std::Binary;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw81_derive::valid_signature_query;

#[valid_signature_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw81QueryMsg {}

#[cw_serde]
pub struct ValidSignatureResponse {
    pub is_valid: bool
}

#[cw_serde]
pub struct ValidSignaturesResponse {
    pub are_valid: Vec<bool>
}