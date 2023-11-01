use cosmwasm_std::{Binary, Empty, CosmosMsg};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw82::{
    smart_account_query, 
    CanExecuteResponse, 
    ValidSignatureResponse, 
    ValidSignaturesResponse, Cw82ExecuteMsg
};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub pubkey: Binary,
    pub token_contract: String,
    pub token_id: String
}

#[cw_serde]
pub struct TokenInfo {
    pub token_contract: String,
    pub token_id: String
}


#[cw_serde]
pub struct SignedMsg<T = Empty> {
    pub msg : CosmosMsg::<T>,
    pub signed_hash : Binary
}

impl<T> From<SignedMsg<T>> for CosmosMsg::<SignedMsg<T>> {
    fn from(msg: SignedMsg<T>) -> Self {
        CosmosMsg::<SignedMsg<T>>::Custom(msg)
    }
}


#[smart_account_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg <T = SignedMsg> {
    #[returns(Binary)]
    Pubkey {},
}

#[cw_serde]
pub struct PayloadInfo {
    pub account: String,
    pub algo: String
}

pub type ExecuteMsg = Cw82ExecuteMsg<Empty>;