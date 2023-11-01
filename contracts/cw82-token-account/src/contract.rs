use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, from_binary, StdError,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cw82::{ValidSignaturesResponse, ValidSignatureResponse, CanExecuteResponse};

use crate::{
    state::{REGISTRY_ADDRESS, TOKEN_INFO, OWNER, PUBKEY}, 
    msg::{QueryMsg, InstantiateMsg, ExecuteMsg, TokenInfo, PayloadInfo}, 
    error::ContractError
};

pub const CONTRACT_NAME: &str = "crates:cw82-token-account";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use k256::sha2::{Digest, Sha256};

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

    if !cw83::Cw83RegistryBase(info.sender.clone()).supports_interface(deps.as_ref())? {
        return Err(ContractError::Unauthorized {})
    };
    
    REGISTRY_ADDRESS.save(deps.storage, &info.sender.to_string())?;
    PUBKEY.save(deps.storage, &msg.pubkey)?;
    OWNER.save(deps.storage, &msg.owner)?;

    TOKEN_INFO.save(deps.storage, &TokenInfo {
        token_contract: msg.token_contract,
        token_id: msg.token_id
    })?;

    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _ : Env, info : MessageInfo, msg : ExecuteMsg) 
-> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Execute { msgs } => {
            let owner = OWNER.load(deps.storage)?;
            if info.sender.to_string() != owner {
                return Err(ContractError::Unauthorized {})
            }
            Ok(Response::new()
                .add_messages(msgs)
            )
        }
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _ : Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pubkey {} => to_binary(&PUBKEY.load(deps.storage)?),

        QueryMsg::CanExecute { sender, .. } => {
            let owner : String = OWNER.load(deps.storage)?;
            to_binary(&CanExecuteResponse { can_execute: owner == sender })
        },

        QueryMsg::ValidSignature { signature, data, payload } => {

            let pk: Binary = PUBKEY.load(deps.storage)?;
            let payload = parse_payload(&payload)?;

            to_binary(&ValidSignatureResponse {
                is_valid: verify_arbitrary(
                    deps,
                    &payload.account,
                    data,
                    signature,
                    &pk
                ).unwrap_or(false)
            })
        },

        QueryMsg::ValidSignatures { signatures, data, payload, .. } => {
            let pk: Binary = PUBKEY.load(deps.storage).unwrap();
            let payload = parse_payload(&payload)?;

            let are_valid : Vec<bool> = signatures
                .into_iter()
                .enumerate()
                .map(|(i, signature)| {

                    let data = data.get(i).unwrap().clone();

                    verify_arbitrary(
                        deps,
                        &payload.account,
                        data,
                        signature,
                        &pk
                    ).unwrap_or(false)

                })
                .collect(); 

            to_binary(&ValidSignaturesResponse {
                are_valid
            })

        }
    }
}


pub fn parse_payload(
    payload: &Option<Binary>
) -> StdResult<PayloadInfo> {

    if payload.is_none() {
        return Err(StdError::GenericErr { 
            msg: "Invalid payload. Must have an 'account' address and 'algo' must be 'amino'".into() 
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


fn generate_amino_transaction_string(signer: &str, data: &str) -> String {
    format!(
        "{{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{{\"amount\":[],\"gas\":\"0\"}},\"memo\":\"\",\"msgs\":[{{\"type\":\"sign/MsgSignData\",\"value\":{{\"data\":\"{}\",\"signer\":\"{}\"}}}}],\"sequence\":\"0\"}}", 
        data, signer
    )
}


pub fn verify_arbitrary(
    deps: Deps,
    account_addr: &str,
    data: Binary,
    signature: Binary,
    pubkey: &[u8],
) -> StdResult<bool> {

    let digest = Sha256::new_with_prefix(generate_amino_transaction_string(
        account_addr,
        from_binary::<String>(&data)?.as_str(),
    )).finalize();

    deps.api.secp256k1_verify(
        &digest, 
        &signature, 
        pubkey
    )?;

    Ok(true)
}

