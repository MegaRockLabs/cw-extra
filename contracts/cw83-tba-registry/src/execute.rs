use cosmwasm_std::{Response, Env, Binary, DepsMut, Coin, SubMsg, ReplyOn, WasmMsg, to_binary, CosmosMsg, Empty, Addr};
use cw83::CREATE_ACCOUNT_REPLY_ID;

use crate::{
    state::{LAST_ATTEMPTING, TOKEN_ADDRESSES, ADMINS},
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
    funds: Vec<Coin>
) -> Result<Response, ContractError> {

    if env.block.chain_id != chain_id {
        return Err(ContractError::InvalidChainId {})
    }

    verify_nft_ownership(deps.as_ref(), &sender, token_info.clone())?;
    LAST_ATTEMPTING.save(deps.storage, &token_info)?;

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


pub fn update_account_owner(
    deps: DepsMut,
    sender: Addr,
    token_info: TokenInfo,
    new_pubkey: Binary,
    funds: Vec<Coin>
) -> Result<Response, ContractError> {
    verify_nft_ownership(deps.as_ref(), sender.as_str(), token_info.clone())?;

    let contract_addr = TOKEN_ADDRESSES.load(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    )?;

    let msg = cw82_token_account::msg::ExecuteMsg::<Empty>::UpdateOwnership { 
        new_owner: sender.to_string(), 
        new_pubkey
    };

    let msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr, 
        msg: to_binary(&msg)?, 
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

    if !ADMINS.load(deps.storage)?.is_admin(sender) {
        return Err(ContractError::Unauthorized {})
    }
    
    let contract_addr = TOKEN_ADDRESSES.load(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    )?;

    let msg = cw82_token_account::msg::ExecuteMsg::<Empty>::Freeze {};

    let msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr, 
        msg: to_binary(&msg)?, 
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

    if !ADMINS.load(deps.storage)?.is_admin(sender) {
        return Err(ContractError::Unauthorized {})
    }
    
    let contract_addr = TOKEN_ADDRESSES.load(
        deps.storage, 
        (token_info.collection.as_str(), token_info.id.as_str())
    )?;

    let msg = cw82_token_account::msg::ExecuteMsg::<Empty>::Unfreeze {};

    let msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr, 
        msg: to_binary(&msg)?, 
        funds: vec![]
    });

    Ok(Response::default()
       .add_message(msg)
    )
}