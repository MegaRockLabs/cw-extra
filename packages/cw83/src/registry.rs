use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, Addr, StdResult, WasmMsg, Coin, Binary, SubMsg, ReplyOn};


pub const CREATE_ACCOUNT_REPLY_ID : u64 = 82;
pub const INTERFACE_NAME: &str = "crates:cw83";



#[cw_serde]
pub struct Cw83RegistryBase (pub Addr);

impl Cw83RegistryBase {
    
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }


    pub fn create_account_init_msg<T>(
        &self, 
        code_id: u64, 
        init_msg: Binary, 
        funds: Vec<Coin>,
        label: String
    ) -> StdResult<CosmosMsg<T>> {

        Ok(
            WasmMsg::Instantiate { 
                admin: Some(self.addr().into()), 
                code_id,
                msg: init_msg,
                funds,
                label
            }.into()
        )
    }

    pub fn create_account_sub_msg<T>(
        &self, 
        code_id: u64, 
        init_msg: Binary, 
        funds: Vec<Coin>,
        label: String,
        payload: Binary
    ) -> StdResult<SubMsg<T>> {

        Ok(SubMsg {
            id: CREATE_ACCOUNT_REPLY_ID,
            msg: self.create_account_init_msg(
                code_id,
                init_msg,
                funds,
                label
            )?, 
            reply_on: ReplyOn::Success,
            gas_limit: None,
            payload,
        })
    }
    

}