use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Empty, Binary};
use cw83_derive::{registy_query, registy_execute};


#[cw_serde]
pub struct AccountInfoResponse<T = Empty> {
    pub address: String,
    pub info: Option<T>
}

#[cw_serde]
pub struct AccountQuery<T = Empty> {
    pub query: T
}

#[cw_serde]
pub struct CreateAccountMsg<T = Binary> {
    pub code_id: u64,
    pub chain_id: String,
    pub msg: T
}


#[registy_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw83QueryMsg {}


#[registy_execute]
#[cw_serde]
pub enum Cw83ExecuteMsg {}