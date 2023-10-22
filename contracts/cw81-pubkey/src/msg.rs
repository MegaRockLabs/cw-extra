use cosmwasm_std::Binary;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw81::{valid_signature_query, ValidSignatureResponse, ValidSignaturesResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub pub_key: Binary
}


#[valid_signature_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    PubKey {},
}

