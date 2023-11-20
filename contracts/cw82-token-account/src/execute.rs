use cosmwasm_std::{
    Deps, DepsMut, Env, Response, CosmosMsg, Addr, Binary, WasmMsg, to_binary, Coin, StdResult, SubMsg, ReplyOn,
};

use cw_ownable::{assert_owner, initialize_owner, is_owner};
use crate::{error::ContractError, utils::{assert_factory, is_ok_cosmos_msg, assert_status}, state::{KNOWN_TOKENS, PUBKEY, STATUS, MINT_CACHE}, msg::Status, contract::MINT_REPLY_ID};




pub fn try_execute(
    deps: Deps,
    sender: Addr,
    msgs: Vec<CosmosMsg>
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &sender)?;
    assert_status(deps.storage)?;
    if !msgs.iter().all(is_ok_cosmos_msg) {
        return Err(ContractError::NotSupported {})
    }
    Ok(Response::new().add_messages(msgs))
}

pub fn try_mint_token(
    deps: DepsMut,
    sender: Addr,
    collection: String,
    funds: Vec<Coin>
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &sender)?;
    assert_status(deps.storage)?;

    MINT_CACHE.save(deps.storage, &collection)?;

    Ok(Response::new().add_submessage(SubMsg {
        msg: WasmMsg::Execute { 
            contract_addr: collection, 
            msg: to_binary(&vending_minter::msg::ExecuteMsg::Mint {})?, 
            funds
        }.into(),
        reply_on: ReplyOn::Success,
        id: MINT_REPLY_ID,
        gas_limit: None,
    }))
}


pub fn try_freeze(
    deps: DepsMut,
    sender: Addr
) -> Result<Response, ContractError> {
    assert_factory(deps.as_ref(), sender)?;
    STATUS.save(deps.storage, &Status { frozen: true })?;
    Ok(Response::default())
}


pub fn try_unfreeze(
    deps: DepsMut,
    sender: Addr,
) -> Result<Response, ContractError> {
    assert_factory(deps.as_ref(), sender)?;
    STATUS.save(deps.storage, &Status { frozen: false })?;
    Ok(Response::default())
}



pub fn try_change_pubkey(
    deps: DepsMut,
    sender: Addr,
    pubkey: Binary
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &sender)?;
    assert_status(deps.storage)?;
    PUBKEY.save(deps.storage, &pubkey)?;
    Ok(Response::default())
}


pub fn try_update_ownership(
    deps: DepsMut,
    sender: Addr,
    new_owner: String,
    new_pubkey: Binary
) -> Result<Response, ContractError> {
    assert_factory(deps.as_ref(), sender)?;
    initialize_owner(deps.storage, deps.api, Some(&new_owner))?;
    STATUS.save(deps.storage, &Status { frozen: false })?;
    PUBKEY.save(deps.storage, &new_pubkey)?;
    Ok(Response::default())
}


pub fn try_forget_tokens(
    deps: DepsMut,
    sender: Addr, 
    contract_addr: String,
    token_ids: Vec<String>
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &sender)?;
    assert_status(deps.storage)?;

    let ids = if token_ids.len() == 0 {
        KNOWN_TOKENS
        .prefix(contract_addr.as_str())
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<StdResult<Vec<String>>>()?

    } else {
        token_ids
    };
    

    for id in ids {
        KNOWN_TOKENS.remove(
            deps.storage, 
            (contract_addr.as_str(), id.as_str()), 
        );
    }

    Ok(Response::default())
}


pub fn try_update_known_tokens(
    deps: DepsMut,
    env: Env,
    sender: Addr,
    contract_addr: String,
    start_after: Option<String>,
    limit: Option<u32>
) -> Result<Response, ContractError> {
    assert_status(deps.storage)?;
    if !is_owner(deps.storage, &sender)? 
        && env.contract.address != sender  {
        return Err(ContractError::Ownership(cw_ownable::OwnershipError::NotOwner))
    }

    let res : cw721::TokensResponse = deps.querier.query_wasm_smart(
        contract_addr.clone(), 
        &sg721_base::msg::QueryMsg::Tokens { 
        owner: env.contract.address.to_string(), 
        start_after, 
        limit 
    })?;

    for id in res.tokens {
        KNOWN_TOKENS.save(
            deps.storage, 
            (contract_addr.as_str(), id.as_str()),
            &true
        )?;
    }

    Ok(Response::default())
}


pub fn try_update_known_on_receive(
    deps: DepsMut,
    collection: String,
    token_id: String,
) -> Result<Response, ContractError> {
    
    KNOWN_TOKENS.save(
        deps.storage, 
        (collection.as_str(), token_id.as_str()),
        &true
    )?;

    Ok(Response::default())
}


pub fn try_transfer_token(
    deps: DepsMut,
    collection: String,
    token_id: String,
    recipient: String,
    funds: Vec<Coin>
) -> Result<Response, ContractError> {
    assert_status(deps.storage)?;
    
    KNOWN_TOKENS.remove(
        deps.storage, 
        (collection.as_str(), token_id.as_str()),
    );

    let msg : CosmosMsg = WasmMsg::Execute { 
        contract_addr: collection, 
        msg: to_binary(&sg721_base::ExecuteMsg::TransferNft { 
            recipient, 
            token_id, 
        })?, 
        funds
    }.into();

    Ok(Response::default().add_message(msg))
}



pub fn try_send_token(
    deps: DepsMut,
    collection: String,
    token_id: String,
    contract: String,
    msg : Binary,
    funds: Vec<Coin>
) -> Result<Response, ContractError> {
    assert_status(deps.storage)?;
    
    KNOWN_TOKENS.remove(
        deps.storage, 
        (collection.as_str(), token_id.as_str()),
    );

    let msg : CosmosMsg = WasmMsg::Execute { 
        contract_addr: collection, 
        msg: to_binary(&sg721_base::ExecuteMsg::SendNft { 
            contract, 
            token_id, 
            msg
        })?, 
        funds
    }.into();

    Ok(Response::default().add_message(msg))
}
