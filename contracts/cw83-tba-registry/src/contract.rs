use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, Reply,
};
use cw82::Cw82Contract;
use cw83::CREATE_ACCOUNT_REPLY_ID;

use crate::{
    state::{STORED_ACCOUNTS, LAST_ATTEMPTING, ALLOWED_IDS},
    msg::{InstantiateMsg, ExecuteMsg}, 
    error::ContractError, execute::create_account, 
};

pub const CONTRACT_NAME: &str = "crates:cw83-tba-factory";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _ : Env, _ : MessageInfo, msg : InstantiateMsg,) 
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

    ALLOWED_IDS.save(deps.storage, &msg.allowed_ids)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env : Env, info : MessageInfo, msg : ExecuteMsg) 
-> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::CreateAccount { 
            code_id, 
            init_msg, 
        } => create_account(
            deps, 
            env,
            info.sender.to_string(),
            code_id, 
            init_msg, 
            vec![]
        )
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
        
        STORED_ACCOUNTS.save(deps.storage, addr.as_str(), &stored)?;
        LAST_ATTEMPTING.remove(deps.storage);

        Ok(Response::default())
    
    } else {
        Err(ContractError::Unauthorized {})
    } 

}