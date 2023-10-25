use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, Addr, StdResult, WasmMsg, Coin, Binary, SubMsg, ReplyOn, Deps, WasmQuery, QueryRequest};


pub const CREATE_ACCOUNT_REPLY_ID : u64 = 82;
pub const INTERFACE_NAME: &str = "crates:cw83";


fn construct_label(
    token_contract: String, 
    token_id: String
) -> String {
    format!("{}-{}", token_contract, token_id)
}



#[cw_serde]
pub struct Cw83RegistryContract(pub Addr);

impl Cw83RegistryContract {
    
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }


    fn create_account_instantiate_msg(
        &self, 
        code_id: u64, 
        init_msg: Binary, 
        token_contract: String, 
        token_id: String,
        funds: Vec<Coin>
    ) -> StdResult<CosmosMsg> {

        Ok(
            WasmMsg::Instantiate { 
                admin: Some(self.addr().into()), 
                code_id,
                msg: init_msg,
                funds: funds,
                label: construct_label(token_contract, token_id)
            }.into()
        )
    }

    pub fn create_account_submsg(
        &self, 
        code_id: u64, 
        init_msg: Binary, 
        token_contract: String, 
        token_id: String,
        funds: Vec<Coin>
    ) -> StdResult<SubMsg> {

        let instant_msg = self.create_account_instantiate_msg(
            code_id,
            init_msg,
            token_contract,
            token_id,
            funds
        )?;

        Ok(SubMsg {
            id: CREATE_ACCOUNT_REPLY_ID,
            msg: instant_msg,
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