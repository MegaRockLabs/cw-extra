use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, BlockInfo, StdError,
};
use cw81::{ValidSignatureResponse, ValidSignaturesResponse};
use cw_utils::Expiration;

use crate::{msg::{QueryMsg, InstantiateMsg, ExecureMsg}, state::{SIGNATURE_STATE, SignatureState}};


pub const CONTRACT_NAME: &str = "crates:cw81-last-signature";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[entry_point]
pub fn instantiate(deps: DepsMut, _ : Env, _ : MessageInfo, _ : InstantiateMsg,) 
-> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    SIGNATURE_STATE.save(deps.storage, &SignatureState {
        signature: Binary::default(),
        expiration: Expiration::Never {},
    })?;
    
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env : Env, _: MessageInfo, msg: ExecureMsg) -> StdResult<Response> {
    match msg {
        ExecureMsg::SaveSignature { signature, expiration } => {
            
            let expiration = expiration.unwrap_or(
                // expires after 100 block by default
                Expiration::AtHeight( env.block.height + 100)
            );

            SIGNATURE_STATE.save(deps.storage, &SignatureState {
                signature,
                expiration,
            })?;

            Ok(Response::default())
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::LastSignature { } => {
            let state = SIGNATURE_STATE.load(deps.storage)?;
            to_binary(&state.signature)
        },

        QueryMsg::ValidSignature { signature, .. } => {
            to_binary(&ValidSignatureResponse {
                is_valid: check_signature_state(deps, &env.block, &signature),
            })
        },

        QueryMsg::ValidSignatures { signatures, .. } => {
            if signatures.len() != 1 {
                return Err(StdError::GenericErr { 
                    msg: String::from("Only one signature is supported") 
                });
            }
            let signature = signatures.get(0).unwrap();
            to_binary(&ValidSignaturesResponse {
                are_valid: vec![check_signature_state(deps, &env.block, &signature)],
            })

        }
    }
}


fn check_signature_state(
    deps: Deps,
    block: &BlockInfo,
    signature: &Binary,
) -> bool {
    let state : SignatureState = SIGNATURE_STATE.load(deps.storage).unwrap();
    !state.expiration.is_expired(block) && state.signature == *signature
}