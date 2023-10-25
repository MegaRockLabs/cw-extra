use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InitTokenAccount {
    token_contract: String,
    token_id: String
}


#[cw_serde]
pub enum Cw83ExecuteMsg<T> {
    CreateAccount {
        code_id: u64,
        init_msg: T,
        token_contract: String,
        token_id: String
    }
}