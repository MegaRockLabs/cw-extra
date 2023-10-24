use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, CosmosMsg,
};
use cw82::{ValidSignaturesResponse, ValidSignatureResponse, CanExecuteResponse};

use crate::{msg::{QueryMsg, InstantiateMsg, ExecuteMsg, SignedMsg}, state::PUBKEY};

pub const CONTRACT_NAME: &str = "crates:cw81-pubkey";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

use sha2::{
    Sha256, 
    digest::{Update, Digest}
};

#[entry_point]
pub fn instantiate(deps: DepsMut, _ : Env, _ : MessageInfo, msg : InstantiateMsg,) 
-> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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
        QueryMsg::PubKey {} => to_binary(&PUBKEY.load(deps.storage)?),

        QueryMsg::CanExecute { msg, .. } => {
            let key: Binary = PUBKEY.load(deps.storage)?;
            
            let can_execute = 
                if let Ok(_) = validate_signed(deps, &msg, &key) {
                    true
                } else {
                    false
                };
            
            to_binary(&CanExecuteResponse { can_execute })
        },


        QueryMsg::ValidSignature { signature, data, .. } => {
            let hash = Sha256::new().chain(&data).finalize();
            let pk: Binary = PUBKEY.load(deps.storage)?;

            to_binary(&ValidSignatureResponse {
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

            to_binary(&ValidSignaturesResponse {
                are_valid
            })

        }
    }
}


fn validate_signed(
    deps: Deps,
    msg: &CosmosMsg<SignedMsg>,
    key: &[u8],
) -> StdResult<CosmosMsg> {


    match msg {
        CosmosMsg::Custom(msg) => {
            let hash = Sha256::new()
                .chain(&to_binary(&msg.msg)?)
                .finalize();
            
            deps.api.secp256k1_verify(
                &hash, 
                &msg.signed_hash, 
                key
            )?;

            Ok(msg.msg.clone())
        },

        _ => Err(cosmwasm_std::StdError::GenericErr { 
            msg: "Only SignedMsg is supported".into() 
        })
        
    }

    

}