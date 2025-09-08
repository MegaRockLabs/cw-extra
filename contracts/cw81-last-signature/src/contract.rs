use types::wasm::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary, BlockInfo, StdError,
};
use cw81::{ValidSignatureResponse, ValidSignaturesResponse};
use cw_utils::Expiration;

use crate::{msg::{QueryMsg, InstantiateMsg, ExecureMsg}, state::{SIGNATURE_STATE, SignatureState}};


pub const CONTRACT_NAME: &str = "crates:cw81-last-signature";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[entry_point]
pub fn instantiate(deps: DepsMut, _ : Env, _ : MessageInfo, _ : InstantiateMsg,) 
-> StdResult<Response> {
    //cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw22::set_contract_supported_interface(
        deps.storage, 
        &[cw22::ContractSupportedInterface {
            supported_interface: cw81::INTERFACE_NAME.into(),
            version: CONTRACT_VERSION.into()
        }]
    )?;
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
            to_json_binary(&state.signature)
        },

        QueryMsg::ValidSignature { signature, .. } => {
            to_json_binary(&ValidSignatureResponse {
                is_valid: check_signature_state(deps, &env.block, signature.to_vec()),
            })
        },

        QueryMsg::ValidSignatures { signatures, .. } => {
            if signatures.len() != 1 {
                return Err(StdError::msg("Only one signature is supported"));
            }
            let signature = signatures.get(0).unwrap();
            to_json_binary(&ValidSignaturesResponse {
                are_valid: vec![check_signature_state(deps, &env.block, signature.to_vec())],
            })

        }
    }
}


fn check_signature_state(
    deps: Deps,
    block: &BlockInfo,
    signature: Vec<u8>
) -> bool {
    let state : SignatureState = SIGNATURE_STATE.load(deps.storage).unwrap();
    !is_expired(state.expiration, block) && state.signature.to_vec() == signature
}


fn is_expired(exp : Expiration, block: &BlockInfo) -> bool {
    match exp {
        Expiration::AtHeight(height) => block.height >= height,
        Expiration::AtTime(time) => block.time.seconds() >= time.seconds(),
        Expiration::Never {} => false,
    }
}