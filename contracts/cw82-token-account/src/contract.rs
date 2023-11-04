use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw_ownable::get_ownership;


use crate::{
    state::{REGISTRY_ADDRESS, TOKEN_INFO, PUBKEY, STATUS}, 
    msg::{QueryMsg, InstantiateMsg, ExecuteMsg, TokenInfo, Status}, 
    error::ContractError, 
    query::{can_execute, valid_signature, valid_signatures, known_tokens, assets}, 
    execute::{try_execute, try_update_ownership, try_update_known_tokens, try_forget_tokens, try_update_known_on_receive, try_transfer_token, try_send_token, try_freeze, try_unfreeze}, 
    utils::is_factory
};
pub const CONTRACT_NAME: &str = "crates:cw82-token-account";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _ : Env, info : MessageInfo, msg : InstantiateMsg) 
-> Result<Response, ContractError> {

    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw22::set_contract_supported_interface(
        deps.storage, 
        &[
            cw22::ContractSupportedInterface {
                supported_interface: cw82::INTERFACE_NAME.into(),
                version: CONTRACT_VERSION.into()
            },
            cw22::ContractSupportedInterface {
                supported_interface: "crates:cw81".into(),
                version: CONTRACT_VERSION.into()
            },
            cw22::ContractSupportedInterface {
                supported_interface: "crates:cw1".into(),
                version: "1.1.1".into()
            },
            cw22::ContractSupportedInterface {
                supported_interface: "crates:cw22".into(),
                // TODO change version
                version: CONTRACT_VERSION.into()
            }
        ]
    )?;

    if !is_factory(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::Unauthorized {})
    };

    cw_ownable::initialize_owner(deps.storage, deps.api, Some(msg.owner.as_str()))?;
    
    TOKEN_INFO.save(deps.storage, &TokenInfo {
        token_contract: msg.token_contract,
        token_id: msg.token_id
    })?;

    REGISTRY_ADDRESS.save(deps.storage, &info.sender.to_string())?;
    STATUS.save(deps.storage, &Status { frozen: false })?;
    PUBKEY.save(deps.storage, &msg.pubkey)?;


    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env : Env, info : MessageInfo, msg : ExecuteMsg) 
-> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Execute { msgs } => try_execute(deps.as_ref(), info.sender, msgs),
        
        ExecuteMsg::TransferToken { 
            collection, 
            token_id, 
            recipient 
        } => try_transfer_token(deps, collection, token_id, recipient, info.funds),

        ExecuteMsg::SendToken { 
            collection, 
            token_id, 
            contract, 
            msg 
        } => try_send_token(deps, collection, token_id, contract, msg, info.funds),


        ExecuteMsg::UpdateKnownTokens { 
            collection, 
            start_after, 
            limit 
        } => try_update_known_tokens(
            deps, 
            env, 
            info.sender, 
            collection, 
            start_after, 
            limit
        ),

        ExecuteMsg::Freeze {} => try_freeze(deps, info.sender),
        
        ExecuteMsg::Unfreeze {} => try_unfreeze(deps, info.sender),

        ExecuteMsg::ForgetTokens { 
            collection, 
            token_ids 
        } => try_forget_tokens(deps, info.sender, collection, token_ids),

        ExecuteMsg::ReceiveNft(
            msg
        ) => try_update_known_on_receive(deps, info.sender.to_string(), msg.token_id),
        
        ExecuteMsg::UpdateOwnership { 
            new_owner, 
            new_pubkey 
        } => try_update_ownership(deps, info.sender, new_owner, new_pubkey)
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env : Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Token {} => to_binary(&TOKEN_INFO.load(deps.storage)?),
        QueryMsg::Status {} => to_binary(&STATUS.load(deps.storage)?),
        QueryMsg::Pubkey {} => to_binary(&PUBKEY.load(deps.storage)?),
        QueryMsg::Assets {} => to_binary(&assets(deps, env)?),
        QueryMsg::Ownership {} => to_binary(&get_ownership(deps.storage)?),
        QueryMsg::CanExecute { sender, .. } => to_binary(&can_execute(deps, sender)?),
        QueryMsg::ValidSignature { 
            signature, 
            data, 
            payload 
        } => to_binary(&valid_signature(deps, data, signature, &payload)?),
        QueryMsg::ValidSignatures { 
            signatures, 
            data, 
            payload 
        } => to_binary(&valid_signatures(deps, data, signatures, &payload)?),
        QueryMsg::KnownTokens {} => to_binary(&known_tokens(deps)?)
    }
}

