use cosmwasm_std::{Response, Env, Binary, DepsMut, Coin, SubMsg, ReplyOn, WasmMsg, to_binary};
use cw83::CREATE_ACCOUNT_REPLY_ID;

use crate::{
    state::LAST_ATTEMPTING,
    helpers::{verify_nft_ownership, construct_label}, 
    error::ContractError, msg::TokenInfo
};

pub fn create_account(
    deps: DepsMut,
    env: Env,
    sender: String,
    code_id: u64,
    token_info: TokenInfo,
    pubkey: Binary,
    funds: Vec<Coin>
) -> Result<Response, ContractError> {

    verify_nft_ownership(deps.as_ref(), &sender, token_info.clone())?;
    LAST_ATTEMPTING.save(deps.storage, &token_info)?;

    let init_msg = cw82_token_account::msg::InstantiateMsg {
        owner: sender.clone(),
        token_contract: token_info.contract.clone(),
        token_id: token_info.id.clone(),
        pubkey
    };


    Ok(Response::default()
       .add_submessage(SubMsg {
            id: CREATE_ACCOUNT_REPLY_ID,
            msg: cosmwasm_std::CosmosMsg::Wasm(
                WasmMsg::Instantiate { 
                    admin: Some(env.contract.address.to_string()), 
                    code_id, 
                    msg: to_binary(&init_msg)?, 
                    funds, 
                    label: construct_label(&token_info) 
                }
            ),
            reply_on: ReplyOn::Success,
            gas_limit: None
        })
    )
}