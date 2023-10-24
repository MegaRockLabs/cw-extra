use cosmwasm_schema::{cw_serde, QueryResponses};


use cosmwasm_std::{CosmosMsg, Binary};
use cw82::*;


#[cw_serde]
pub struct  EncryptedMsg {
    pub encrypted_msg : Binary
}

impl From<EncryptedMsg> for CosmosMsg::<EncryptedMsg> {
    fn from(msg: EncryptedMsg) -> Self {
        CosmosMsg::<EncryptedMsg>::Custom(msg)
    }
}


#[extended_smart_account_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg <T = EncryptedMsg> {
    #[returns(Binary)]
    Signature { to_sign : Binary },

}


#[cw_serde]
pub struct InstantiateMsg {
    pub secret_key : Binary
}
