use cosmwasm_std::{Response, Env, Binary, DepsMut, Coin, SubMsg, ReplyOn, WasmMsg, to_json_binary, CosmosMsg, Addr};
use cw83::CREATE_ACCOUNT_REPLY_ID;

use crate::{
    state::{TOKEN_ADDRESSES, ADMINS, ALLOWED_IDS},
    helpers::{verify_nft_ownership, construct_label}, 
    error::ContractError, msg::TokenInfo
};

pub fn create_account(
    deps: DepsMut,
    env: Env,
    sender: String,
    chain_id: String,
    code_id: u64,
    token_info: TokenInfo,
    pubkey: Binary,
    funds: Vec<Coin>,
    reset: bool
) -> Result<Response, ContractError> {

    if env.block.chain_id != chain_id {
        return Err(ContractError::InvalidChainId {})
    }

    if !ALLOWED_IDS.load(deps.storage)?.contains(&code_id) {
        return Err(ContractError::InvalidCodeId {})
    }

    verify_nft_ownership(deps.as_ref(), &sender, token_info.clone())?;

    if !reset && TOKEN_ADDRESSES.has(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    ) {
        return Err(ContractError::AccountExists {})
    }


    let init_msg = cw82_token_account::msg::InstantiateMsg {
        owner: sender.clone(),
        token_contract: token_info.collection.clone(),
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
                    msg: to_json_binary(&init_msg)?, 
                    funds, 
                    label: construct_label(&token_info) 
                }
            ),
            reply_on: ReplyOn::Success,
            gas_limit: None,
            payload: to_json_binary(&token_info)?,
        })
    )
}


pub fn update_account_owner(
    deps: DepsMut,
    sender: Addr,
    token_info: TokenInfo,
    new_pubkey: Binary,
    funds: Vec<Coin>,
    update_for: Option<Addr>
) -> Result<Response, ContractError> {

    let owner = if update_for.is_some () {
        if !ADMINS.load(deps.storage)?.is_admin(sender.as_ref()) {
            return Err(ContractError::Unauthorized {})
        }
        update_for.unwrap()
    } else {
        sender.clone()
    };

    verify_nft_ownership(deps.as_ref(), owner.as_str(), token_info.clone())?;

    let contract_addr = TOKEN_ADDRESSES.load(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    )?;

    let msg = cw82_token_account::msg::ExecuteMsg::UpdateOwnership { 
        new_owner: owner.to_string(), 
        new_pubkey
    };

    let msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr, 
        msg: to_json_binary(&msg)?, 
        funds 
    });

    Ok(Response::default()
       .add_message(msg)
    )
}


pub fn freeze_account(
    deps: DepsMut,
    sender: Addr,
    token_info: TokenInfo,
) -> Result<Response, ContractError> {

    if !ADMINS.load(deps.storage)?.is_admin(sender.as_ref()) {
        return Err(ContractError::Unauthorized {})
    }
    
    let contract_addr = TOKEN_ADDRESSES.load(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    )?;

    let msg = cw82_token_account::msg::ExecuteMsg::Freeze {};

    let msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr, 
        msg: to_json_binary(&msg)?, 
        funds: vec![]
    });

    Ok(Response::default()
       .add_message(msg)
    )
}


pub fn unfreeze_account(
    deps: DepsMut,
    sender: Addr,
    token_info: TokenInfo,
) -> Result<Response, ContractError> {

    if !ADMINS.load(deps.storage)?.is_admin(sender.as_ref()) {
        return Err(ContractError::Unauthorized {})
    }
    
    let contract_addr = TOKEN_ADDRESSES.load(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    )?;

    let msg = cw82_token_account::msg::ExecuteMsg::Unfreeze {};

    let msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr, 
        msg: to_json_binary(&msg)?, 
        funds: vec![]
    });

    Ok(Response::default()
       .add_message(msg)
    )
}



pub fn migrate_account(
    deps: DepsMut,
    sender: Addr,
    token_info: TokenInfo,
    new_code_id: u64,
    msg: Binary
) -> Result<Response, ContractError> {

    if !ALLOWED_IDS.load(deps.storage)?.contains(&new_code_id) {
        return Err(ContractError::InvalidCodeId {});
    }

    verify_nft_ownership(deps.as_ref(), sender.as_str(), token_info.clone())?;

    let contract_addr = TOKEN_ADDRESSES.load(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    )?;

    let msg = CosmosMsg::Wasm(WasmMsg::Migrate { 
        contract_addr, 
        new_code_id, 
        msg 
    });
    

    Ok(Response::default()
       .add_message(msg)
    )
}