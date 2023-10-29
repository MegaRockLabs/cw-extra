use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InitTokenAccount {
    token_contract: String,
    token_id: String
}


#[cw_serde]
pub enum Cw83ExecuteMsg {
    CreateAccount {
        code_id: u64,
        init_msg: Binary
    }
}