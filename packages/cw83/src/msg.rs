use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Empty, Binary};


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


#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw83QueryMsg {
    #[returns(AccountInfoResponse)]
    AccountInfo(AccountQuery)
}


#[cw_serde]
pub enum Cw83ExecuteMsg {
    CreateAccount(CreateAccountMsg)
}