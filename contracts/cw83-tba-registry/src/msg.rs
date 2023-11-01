use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw83::registy_execute;

#[cw_serde]
pub struct InstantiateMsg {
    pub allowed_ids: Vec<u64>,
}


#[registy_execute]
#[cw_serde]
pub enum ExecuteMsg {
    UpdateAllowedIds {
        allowed_ids: Vec<u64>
    },
}