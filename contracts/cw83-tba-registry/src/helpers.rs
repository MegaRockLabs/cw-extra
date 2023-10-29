use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, to_binary, Binary, from_binary, Addr, Coin, StdResult, CosmosMsg, SubMsg, ReplyOn, StdError};
use cw83::{Cw83RegistryBase, CREATE_ACCOUNT_REPLY_ID};

use crate::error::ContractError;


pub fn construct_label(
    token_contract: &str, 
    token_id: &str
) -> String {
    format!("{}-{}-account", token_contract, token_id)
}


pub fn verify_nft_ownership(
    deps: Deps,
    sender: &String,
    token_contract: String,
    token_id : String,
) -> Result<(), ContractError> {

    let owner_res = deps
        .querier
        .query_wasm_smart::<cw721::OwnerOfResponse>(
        token_contract, 
        &sg721_base::QueryMsg::OwnerOf {
            token_id,
            include_expired: None
        }
    )?;

    if owner_res.owner != *sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}



#[cw_serde]
pub struct Cw83TokenRegistryContract(pub Addr);

impl Cw83TokenRegistryContract {
    
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    fn cw83_wrap(&self) -> Cw83RegistryBase {
        Cw83RegistryBase(self.addr())
    }

    fn init_binary(
        &self,
        owner: String,
        pubkey: Binary,
        token_contract: String, 
        token_id: String,
    ) -> StdResult<Binary> {

        let msg = cw82_token_account::msg::InstantiateMsg {
            owner,
            pubkey,
            token_contract: token_contract.clone(),
            token_id: token_id.clone(),
        };
        

        to_binary(&msg)
    }

    pub fn create_account_init_msg(
        &self, 
        code_id: u64, 
        owner: String,
        pubkey: Binary,
        token_contract: String, 
        token_id: String,
        funds: Vec<Coin>
    ) -> StdResult<CosmosMsg> {

        self.cw83_wrap().create_account_init_msg(
            code_id,
            self.init_binary(
                owner,
                pubkey,
                token_contract.clone(),
                token_id.clone(),
            )?,
            funds,
            construct_label(&token_contract, &token_id)
        )
    }

    pub fn create_account_sub_msg(
        &self, 
        code_id: u64, 
        owner: String,
        pubkey: Binary,
        token_contract: String, 
        token_id: String,
        funds: Vec<Coin>
    ) -> StdResult<SubMsg> {

        Ok(SubMsg {
            id: CREATE_ACCOUNT_REPLY_ID,
            msg: self.create_account_init_msg(
                code_id,
                owner,
                pubkey,
                token_contract,
                token_id,
                funds
            )?,
            reply_on: ReplyOn::Success,
            gas_limit: None
        })
    }
    
    pub fn supports_interface(
        &self,
        deps: Deps,
    ) -> StdResult<bool> {
        self.cw83_wrap().supports_interface(deps)
    }

}