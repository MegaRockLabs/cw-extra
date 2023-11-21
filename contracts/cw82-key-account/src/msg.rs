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
    pub pub_key: Binary
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
    PubKey {},
}


pub type ExecuteMsg = Cw82ExecuteMsg<SignedMsg>;