use cosmwasm_std::{
    Deps, DepsMut, Env, Response, CosmosMsg, Addr, Binary, WasmMsg, to_binary, Coin,
};

use cw_ownable::{assert_owner, initialize_owner};
use crate::{error::ContractError, utils::is_factory, state::{KNOWN_TOKENS, PUBKEY, STATUS}, msg::Status};


pub fn try_execute(
    deps: Deps,
    sender: Addr,
    msgs: Vec<CosmosMsg>
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &sender)?;
    Ok(Response::new().add_messages(msgs))
}


pub fn try_freeze(
    deps: DepsMut,
    sender: Addr,
    //to_revoke: Option<Vec<String>>
) -> Result<Response, ContractError> {
    if !is_factory(deps.as_ref(), sender)? {
        return Err(ContractError::Unauthorized {})
    }
    STATUS.save(deps.storage, &Status { frozen: true })?;
    Ok(Response::default())
}

pub fn try_unfreeze(
    deps: DepsMut,
    sender: Addr,
) -> Result<Response, ContractError> {
    if !is_factory(deps.as_ref(), sender)? {
        return Err(ContractError::Unauthorized {})
    }
    STATUS.save(deps.storage, &Status { frozen: false })?;
    Ok(Response::default())
}



pub fn try_change_pubkey(
    deps: DepsMut,
    sender: Addr,
    pubkey: Binary
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &sender)?;
    PUBKEY.save(deps.storage, &pubkey)?;
    Ok(Response::default())
}


pub fn try_update_ownership(
    deps: DepsMut,
    sender: Addr,
    new_owner: String,
    new_pubkey: Binary
) -> Result<Response, ContractError> {
    if !is_factory(deps.as_ref(), sender)? {
        return Err(ContractError::Unauthorized {})
    }
    initialize_owner(deps.storage, deps.api, Some(&new_owner))?;
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

    for id in token_ids {
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
    assert_owner(deps.storage, &sender)?;

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


/* fn revoke_approvals_msgs(
    deps: Deps,
    sender: &str,
    to_revoke: Vec<String>
 ) -> StdResult<Vec<CosmosMsg>> {

    let mut msgs : Vec<CosmosMsg> = vec![];

    for collection in to_revoke {

        cw721::Cw721Query::approvals(&self, deps, env, token_id, include_expired)

        sg721_base::ExecuteMsg::RevokeAll { operator: () }
        msgs.push(
            WasmMsg::Execute {
                contract_addr: sender.to_string(),
                msg: to_binary(&sg721_base::ExecuteMsg::RevokeApproval { 
                    token_id: token_id.clone()
                })?,
                funds: vec![]
            }.into()
        );
    }


    Ok(msgs)
} */