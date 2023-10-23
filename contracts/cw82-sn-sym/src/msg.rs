use cosmwasm_std::StdError;
use cosmwasm_schema::{cw_serde, QueryResponses};
//use cw82::Cw82ExtendedQueryMsg;
use thiserror::Error;


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


// pub type QueryMsg = Cw82ExtendedQueryMsg;


#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
