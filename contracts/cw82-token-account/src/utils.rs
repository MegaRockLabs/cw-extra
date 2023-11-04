use cosmwasm_std::{Addr, Deps, StdResult, Binary, StdError, from_binary};

use crate::msg::PayloadInfo;

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