use cosmwasm_std::{
    to_binary, DepsMut, Deps, Env, MessageInfo, Response, Reply, StdResult, Binary
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cw82::Cw82Contract;
use cw83::CREATE_ACCOUNT_REPLY_ID;

use crate::{
    state::{LAST_ATTEMPTING, ALLOWED_IDS, TOKEN_ADDRESSES, ADMINS, AdminList, COL_TOKEN_COUNTS},
    msg::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg}, 
    error::ContractError, execute::{create_account, update_account_owner, freeze_account, unfreeze_account, migrate_account}, query::{account_info, accounts, collections, collection_accounts}, 
};

pub const CONTRACT_NAME: &str = "crates:cw83-tba-registry";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _ : Env, info : MessageInfo, msg : InstantiateMsg,) 
-> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw22::set_contract_supported_interface(
        deps.storage, 
        &[
            cw22::ContractSupportedInterface {
                supported_interface: cw83::INTERFACE_NAME.into(),
                version: CONTRACT_VERSION.into()
            }
        ]
    )?;

    ADMINS.save(deps.storage, &AdminList {
        admins: msg.admins.unwrap_or(vec![info.sender])
    })?;
    ALLOWED_IDS.save(deps.storage, &msg.allowed_ids)?;

    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env : Env, info : MessageInfo, msg : ExecuteMsg) 
-> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::CreateAccount(
            create
        ) => create_account(
            deps, 
            env,
            info.sender.to_string(),
            create.chain_id,
            create.code_id, 
            create.msg.token_info, 
            create.msg.pubkey,
            info.funds,
            false
        ),

        ExecuteMsg::ResetAccount(
            create
        ) => create_account(
            deps, 
            env,
            info.sender.to_string(),
            create.chain_id,
            create.code_id, 
            create.msg.token_info, 
            create.msg.pubkey,
            info.funds,
            true
        ),

        ExecuteMsg::MigrateAccount { 
            token_info,
            new_code_id,
            msg
        } => migrate_account(deps, info.sender, token_info, new_code_id, msg),
        
        ExecuteMsg::UpdateAllowedIds { 
            allowed_ids 
        } => {
            if !ADMINS.load(deps.storage)?.is_admin(info.sender.as_ref()) {
                return Err(ContractError::Unauthorized {})
            }
            ALLOWED_IDS.save(deps.storage, &allowed_ids)?;
            Ok(Response::default())
        },

        ExecuteMsg::UpdateAccountOwnership { 
            token_info, 
            new_pubkey ,
            update_for
        } => update_account_owner(
            deps, 
            info.sender, 
            token_info, 
            new_pubkey, 
            info.funds,
            update_for
        ),

        ExecuteMsg::FreezeAccount { token_info } => freeze_account(deps, info.sender, token_info),
        
        ExecuteMsg::UnfreezeAccount { token_info } => unfreeze_account(deps, info.sender, token_info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _ : Env, msg : Reply) 
-> Result<Response, ContractError> {

    if msg.id == CREATE_ACCOUNT_REPLY_ID {
        let res = cw_utils::parse_reply_instantiate_data(msg)?;

        let addr = res.contract_address;
        let ver_addr = deps.api.addr_validate(addr.as_str())?;

        Cw82Contract(ver_addr).supports_interface(deps.as_ref())?;
        
        let stored = LAST_ATTEMPTING.load(deps.storage)?;
        LAST_ATTEMPTING.remove(deps.storage);


        COL_TOKEN_COUNTS.update(
            deps.storage, 
            stored.collection.as_str(), 
            |count| -> StdResult<u32> {
                match count {
                    Some(c) => Ok(c+1),
                    None => Ok(1)
                }
            }
        )?;

        TOKEN_ADDRESSES.save(
            deps.storage, 
            (stored.collection.as_str(), stored.id.as_str()), 
            &addr        
        )?;

        Ok(Response::default())
    
    } else {
        Err(ContractError::Unauthorized {})
    } 

}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _ : Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AccountInfo(
            acc_query
        ) => to_binary(&account_info(deps, acc_query.query)?),

        QueryMsg::Collections {
            skip,
            limit
        } => to_binary(&collections(deps, skip, limit)?),

        QueryMsg::Accounts { 
            skip, 
            limit 
        } => to_binary(&accounts(
            deps, 
            skip,
            limit
        )?),

        QueryMsg::CollectionAccounts { 
            collection, 
            skip, 
            limit 
        } => to_binary(&collection_accounts(
            deps, 
            &collection,
            skip,
            limit
        )?)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_: DepsMut, _: Env, _: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}