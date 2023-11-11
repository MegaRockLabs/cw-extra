use cosmwasm_std::{Addr, Deps, StdResult, Binary, StdError, from_binary, CosmosMsg, WasmMsg, Storage};
use crate::{msg::PayloadInfo, error::ContractError, state::STATUS};

pub fn assert_status(
    store: &dyn Storage
) -> StdResult<bool>{
    let status =STATUS.load(store)?;
    Ok(!status.frozen)
}   

pub fn status_ok(
    store: &dyn Storage
) -> bool {
    assert_status(store).is_ok()
}


pub fn assert_ok_wasm_msg(
    msg: &WasmMsg
) -> StdResult<()> {
    let bad_wasm_error  = StdError::GenericErr { msg: "Not Supported".into() };
    match msg {
        // todo: add whitelististed messages
        WasmMsg::Execute { .. } => Err(bad_wasm_error),
        _ => Err(bad_wasm_error)
    }
}


pub fn assert_ok_cosmos_msg(
    msg: &CosmosMsg
) -> StdResult<()> {
    let bad_msg_error = StdError::GenericErr { msg: "Not Supported".into() };
    match msg {
        CosmosMsg::Wasm(msg) => assert_ok_wasm_msg(msg),
        CosmosMsg::Custom(_) => Err(bad_msg_error),
        CosmosMsg::Stargate { .. } => Err(bad_msg_error),
        _ => Ok(())
    }
}

pub fn is_ok_cosmos_msg(
    msg: &CosmosMsg
) -> bool {
    assert_ok_cosmos_msg(msg).is_ok()
}


pub fn assert_factory(
    deps: Deps,
    addr: Addr
) -> Result<(), ContractError> {
    if is_factory(deps, addr)? {
        Ok(())
    } else {
        Err(ContractError::Unauthorized {})
    }
}

pub fn is_factory(
    deps: Deps,
    addr: Addr
) -> StdResult<bool> {
    cw83::Cw83RegistryBase(addr).supports_interface(deps)
}

pub fn parse_payload(
    payload: &Option<Binary>
) -> StdResult<PayloadInfo> {

    if payload.is_none() {
        return Err(StdError::GenericErr { 
            msg: "Invalid payload. Must have an 'account' address and 'algo' must be 'amino_direct'".into() 
        })
    }

    let payload : PayloadInfo = from_binary(payload.as_ref().unwrap())?;
    
    if payload.account.len() < 1 || payload.algo != "amino" {
        return Err(StdError::GenericErr { 
            msg: "Invalid payload. Must have an 'account' address and 'algo' must be 'amino'".into() 
        })
    }

    Ok(payload)
}


pub fn generate_amino_transaction_string(signer: &str, data: &str) -> String {
    format!(
        "{{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{{\"amount\":[],\"gas\":\"0\"}},\"memo\":\"\",\"msgs\":[{{\"type\":\"sign/MsgSignData\",\"value\":{{\"data\":\"{}\",\"signer\":\"{}\"}}}}],\"sequence\":\"0\"}}", 
        data, signer
    )
}