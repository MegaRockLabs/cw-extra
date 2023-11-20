use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Empty, Reply,
};
use cw_ownable::{get_ownership, initialize_owner};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;


use crate::{
    state::{REGISTRY_ADDRESS, TOKEN_INFO, PUBKEY, STATUS, MINT_CACHE}, 
    msg::{QueryMsg, InstantiateMsg, ExecuteMsg, TokenInfo, Status, MigrateMsg}, 
    error::ContractError, 
    query::{can_execute, valid_signature, valid_signatures, known_tokens, assets, full_info}, 
    execute::{try_execute, try_update_ownership, try_update_known_tokens, try_forget_tokens, try_update_known_on_receive, try_transfer_token, try_send_token, try_freeze, try_unfreeze, try_change_pubkey, try_mint_token}, 
};

#[cfg(target_arch = "wasm32")]
use crate::utils::is_factory;

pub const CONTRACT_NAME: &str = "crates:cw82-token-account";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MINT_REPLY_ID: u64 = 1;


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

    #[cfg(target_arch = "wasm32")]
    if !is_factory(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::Unauthorized {})
    };

    initialize_owner(deps.storage, deps.api, Some(msg.owner.as_str()))?;
    
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

        ExecuteMsg::MintToken { collection } => try_mint_token(
            deps,
            info.sender,
            collection, 
            info.funds
        ),
        
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
        } => try_update_ownership(deps, info.sender, new_owner, new_pubkey),

        ExecuteMsg::UpdatePubkey { new_pubkey } => try_change_pubkey(deps, info.sender, new_pubkey),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env : Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Token {} => to_binary(&TOKEN_INFO.load(deps.storage)?),
        QueryMsg::Status {} => to_binary(&STATUS.load(deps.storage)?),
        QueryMsg::Pubkey {} => to_binary(&PUBKEY.load(deps.storage)?),
        QueryMsg::Registry {} => to_binary(&REGISTRY_ADDRESS.load(deps.storage)?),
        QueryMsg::Ownership {} => to_binary(&get_ownership(deps.storage)?),
        QueryMsg::CanExecute { 
            sender, 
            msg 
        } => to_binary(&can_execute(deps, sender, &msg)?),
        QueryMsg::ValidSignature { 
            signature, 
            data, 
            payload ,
        } => to_binary(&valid_signature(deps, data, signature, &payload)?),
        QueryMsg::ValidSignatures { 
            signatures, 
            data, 
            payload 
        } => to_binary(&valid_signatures(deps, data, signatures, &payload)?),
        QueryMsg::KnownTokens {
            skip,
            limit
        } => to_binary(&known_tokens(deps, skip, limit)?),
        QueryMsg::Assets {
            skip,
            limit
        } => to_binary(&assets(deps, env, skip, limit)?),
        QueryMsg::FullInfo {
            skip,
            limit
        } => to_binary(&full_info(deps, env, skip, limit)?)

    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _: Env, _: MigrateMsg<Empty>) -> StdResult<Response> {
    STATUS.save(deps.storage, &Status { frozen: false })?;
    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        MINT_REPLY_ID => {
            let collection = MINT_CACHE.load(deps.storage)?;
            MINT_CACHE.remove(deps.storage);

            try_update_known_tokens(
                deps, 
                env.clone(), 
                env.contract.address, 
                collection.to_string(), 
                None, 
                None
            )
        }

        _ => Err(ContractError::NotSupported {})
    }
}