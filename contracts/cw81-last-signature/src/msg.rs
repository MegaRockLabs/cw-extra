use cosmwasm_std::Binary;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw81::valid_signature_query;
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecureMsg {
    SaveSignature {
        signature: Binary,
        expiration: Option<Expiration>,
    },
}

#[valid_signature_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    LastSignature {},
}

