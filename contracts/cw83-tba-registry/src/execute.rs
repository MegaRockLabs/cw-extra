use cosmwasm_std::{Response, Env, Binary, DepsMut, Coin, from_binary, SubMsg, ReplyOn, WasmMsg, StdError};
use cw83::CREATE_ACCOUNT_REPLY_ID;

use crate::{
    state::{LAST_ATTEMPTING, StoredAccount},
    helpers::{verify_nft_ownership, construct_label}, 
    error::ContractError
};

pub fn create_account(
    deps: DepsMut,
    env: Env,
    sender: String,
    code_id: u64,
    init_msg: Binary,
    funds: Vec<Coin>
) -> Result<Response, ContractError> {

    let decoded : cw82_token_account::msg::InstantiateMsg  = from_binary(&init_msg)?;

    let token_contract = decoded.token_contract;
    let token_id = decoded.token_id;

    verify_nft_ownership(deps.as_ref(), &sender, token_contract.clone(), token_id.clone())?;

    LAST_ATTEMPTING.save(deps.storage, &StoredAccount{
        owner: sender,
        token_contract: token_contract.clone(),
        token_id: token_id.clone()
    })?;

    Ok(Response::default()
       .add_submessage(SubMsg {
            id: CREATE_ACCOUNT_REPLY_ID,
            msg: cosmwasm_std::CosmosMsg::Wasm(
                WasmMsg::Instantiate { 
                    admin: Some(env.contract.address.to_string()), 
                    code_id, 
                    msg: init_msg, 
                    funds, 
                    label: construct_label(&token_contract, &token_id) 
                }
            ),
            reply_on: ReplyOn::Success,
            gas_limit: None
        })
    )
}