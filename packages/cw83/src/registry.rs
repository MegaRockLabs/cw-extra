use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, Addr, StdResult, WasmMsg, Coin, Binary, SubMsg, ReplyOn, Deps, WasmQuery, QueryRequest};


pub const CREATE_ACCOUNT_REPLY_ID : u64 = 82;
pub const INTERFACE_NAME: &str = "crates:cw83";


/* trait Cw83Registry<T> {
    fn addr(&self) -> Addr;
    fn create_account_init_msg(
        &self, 
        code_id: u64, 
        init_msg: Binary, 
        funds: Vec<Coin>,
        label: String
    ) -> StdResult<CosmosMsg<T>>;
    fn create_account_sub_msg(
        &self,
        code_id: u64, 
        init_msg: Binary, 
        funds: Vec<Coin>,
        label: String
    ) -> StdResult<SubMsg<T>>;
}
 */

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
        label: String
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
            gas_limit: None
        })
    }
    
    pub fn supports_interface(
        &self,
        deps: Deps,
    ) -> StdResult<bool> {

        let key = cosmwasm_std::storage_keys::namespace_with_key(
            &[cw22::SUPPORTED_INTERFACES.namespace()], 
            INTERFACE_NAME.as_bytes()
        );

        let raw_query = WasmQuery::Raw { 
            contract_addr: self.addr().into(),
            key: key.into()
        };

        let version : Option<String> = deps.querier.query(&QueryRequest::Wasm(raw_query))?;
        Ok(version.is_some())
    }

}