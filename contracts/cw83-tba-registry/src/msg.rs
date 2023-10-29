use cosmwasm_schema::cw_serde;
use cw83::Cw83ExecuteMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub allowed_ids: Vec<u64>,
}

pub type ExecuteMsg = Cw83ExecuteMsg;