use types::wasm::{
    entry_point, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult
};
use cw82::{ValidSignaturesResponse, ValidSignatureResponse, CanExecuteResponse};
use crate::{msg::{QueryMsg, InstantiateMsg, ExecuteMsg, SignedMsg}, state::PUBKEY};

pub const CONTRACT_NAME: &str = "crates:cw82-key-account";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use sha2::{
    Sha256, 
    digest::{Update, Digest}
};

#[entry_point]
pub fn instantiate(deps: DepsMut, _ : Env, _ : MessageInfo, msg : InstantiateMsg,) 
-> StdResult<Response> {
    //cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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
    PUBKEY.save(deps.storage, &msg.pub_key)?;
    Ok(Response::default())
}


#[entry_point]
pub fn execute(deps: DepsMut, _ : Env, _ : MessageInfo, msg : ExecuteMsg) 
-> StdResult<Response> {
    match msg {
        ExecuteMsg::Execute { msgs } => {
            let key: Binary = PUBKEY.load(deps.storage)?;
            let msgs : StdResult<Vec<CosmosMsg>, > = msgs
                .iter()
                .map(|msg| 
                    validate_signed(deps.as_ref(), msg, &key)
                )
                .collect();
            Ok(Response::new()
                .add_messages(msgs.unwrap())
            )
        }
    }
}


#[entry_point]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PubKey {} => to_json_binary(&PUBKEY.load(deps.storage)?),

        QueryMsg::CanExecute { msg, .. } => {
            let key: Binary = PUBKEY.load(deps.storage)?;
            let can_execute = 
                if let Ok(_) = validate_signed(deps, &msg, &key) {
                    true
                } else {
                    false
                };
            to_json_binary(&CanExecuteResponse { can_execute })
        },


        QueryMsg::ValidSignature { signature, data, .. } => {
            let hash = Sha256::new().chain(&data).finalize();
            let pk: Binary = PUBKEY.load(deps.storage)?;

            to_json_binary(&ValidSignatureResponse {
                is_valid: deps.api.secp256k1_verify(
                    &hash, 
                    &signature, 
                    &pk
                ).unwrap_or(false),
            })
        },

        QueryMsg::ValidSignatures { signatures, data, .. } => {
            let pk: Binary = PUBKEY.load(deps.storage).unwrap();

            let are_valid : Vec<bool> = signatures
                .iter()
                .enumerate()
                .map(|(i, signature)| {

                    let hash = Sha256::new().chain(
                        data.get(i).unwrap()
                    ).finalize();

                    deps.api.secp256k1_verify(
                        &hash, 
                        &signature, 
                        &pk
                    ).unwrap_or(false)

                })
                .collect(); 

            to_json_binary(&ValidSignaturesResponse {
                are_valid
            })

        }
    }
}


fn validate_signed(
    deps: Deps,
    msg: &cosmwasm_std::CosmosMsg<SignedMsg>,
    key: &[u8],
) -> StdResult<CosmosMsg> {


    match msg {
        cosmwasm_std::CosmosMsg::Custom(msg) => {
            let hash = Sha256::new()
                .chain(&to_json_binary(&msg.msg)?)
                .finalize();
            
            deps.api.secp256k1_verify(
                &hash, 
                &msg.signed_hash, 
                key
            )?;

            Ok(msg.msg.clone())
        },

        _ => Err(StdError::generic_err("Only SignedMsg is supported"))
        
    }

    

}