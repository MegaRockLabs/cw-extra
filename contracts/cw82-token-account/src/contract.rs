use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary,
};
use cw82::{ValidSignaturesResponse, ValidSignatureResponse, CanExecuteResponse};

use crate::{
    state::{REGISTRY_ADDRESS, TOKEN_INFO, OWNER, PUBKEY}, 
    msg::{QueryMsg, InstantiateMsg, ExecuteMsg, TokenInfo}, 
    error::ContractError
};

pub const CONTRACT_NAME: &str = "crates:cw82-token-account";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use sha2::{
    Sha256, 
    digest::{Update, Digest}
};

#[entry_point]
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

    if !cw83::Cw83RegistryContract(info.sender.clone()).supports_interface(deps.as_ref())? {
        return Err(ContractError::Unauthorized {})
    };
    
    REGISTRY_ADDRESS.save(deps.storage, &info.sender.to_string())?;
    OWNER.save(deps.storage, &msg.owner)?;

    TOKEN_INFO.save(deps.storage, &TokenInfo {
        token_contract: msg.token_contract,
        token_id: msg.token_id
    })?;

    Ok(Response::default())
}


#[entry_point]
pub fn execute(deps: DepsMut, _ : Env, info : MessageInfo, msg : ExecuteMsg) 
-> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Execute { msgs } => {
            let owner = OWNER.load(deps.storage)?;
            if info.sender.to_string() == owner {
                return Err(ContractError::Unauthorized {})
            }
            Ok(Response::new()
                .add_messages(msgs)
            )
        }
    }
}


#[entry_point]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PubKey {} => to_binary(&PUBKEY.load(deps.storage)?),

        QueryMsg::CanExecute { sender, .. } => {
            let owner : String = OWNER.load(deps.storage)?;
            to_binary(&CanExecuteResponse { can_execute: owner == sender })
        },

        QueryMsg::ValidSignature { signature, data, payload } => {

            let pk: Binary = PUBKEY.load(deps.storage)?;
            
            let hash: Vec<u8> = if payload.is_some() {
                Sha256::new().chain(&data).chain(&data).finalize().as_slice().into()
            } else {
                data.into()
            };

            to_binary(&ValidSignatureResponse {
                is_valid: deps.api.secp256k1_verify(
                    &hash, 
                    &signature, 
                    &pk
                ).unwrap_or(false),
            })
        },

        QueryMsg::ValidSignatures { signatures, data, payload, .. } => {
            let pk: Binary = PUBKEY.load(deps.storage).unwrap();

            let are_valid : Vec<bool> = signatures
                .iter()
                .enumerate()
                .map(|(i, signature)| {

                    let data = data.get(i).unwrap().clone();

                    let hash: Vec<u8> = if payload.is_some() {
                        Sha256::new().chain(&data).chain(&data).finalize().as_slice().into()
                    } else {
                        data.into()
                    };

                    deps.api.secp256k1_verify(
                        &hash, 
                        &signature, 
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
