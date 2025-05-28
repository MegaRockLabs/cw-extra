use types::wasm::{Binary, Empty, CosmosMsg};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw82::{account_query, account_execute};

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

#[account_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<T = SignedMsg> {
    #[returns(Binary)]
    PubKey {},
}


#[account_execute]
#[cw_serde]
pub enum ExecuteMsg<T = SignedMsg> {}