
use cosmwasm_std::{entry_point, Response, DepsMut, MessageInfo, Env, StdResult, Binary, Deps, to_binary, CosmosMsg, from_binary};
use cw82::{ValidSignatureResponse, ValidSignaturesResponse, CanExecuteResponse};
use cw2::ContractVersion;

use crate::{
    msg::{InstantiateMsg, QueryMsg, EncryptedMsg, ExecuteMsg}, 
    state::{save_private, read_private, KeyType}
};


use k256::ecdsa::{signature::DigestSigner, SigningKey, Signature};
use ecies::{SecretKey, symmetric::sym_decrypt};


use rand_chacha::{
    ChaChaRng, 
    rand_core::SeedableRng
};


pub const CONTRACT_NAME: &str = "crates:cw82-sn-ks";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


use sha2::{
    Sha256, 
    digest::{Update, Digest}
};

#[entry_point]
pub fn instantiate(deps: DepsMut, env : Env, _ : MessageInfo, msg : InstantiateMsg,) 
-> StdResult<Response> {
    let mut ring = ChaChaRng::from_seed(env.block.random.unwrap().to_array()?);
    let secret_key = SigningKey::random(&mut ring);
    let secret_binary : Binary = secret_key.to_bytes().as_slice().into();

    save_private(deps.storage, &secret_binary, KeyType::Signing)?;
    save_private(deps.storage, &msg.secret_key, KeyType::Decrypting)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, _ : Env, _ : MessageInfo, msg : ExecuteMsg) 
-> StdResult<Response> {

    match msg {
        ExecuteMsg::Execute { msgs } => {

            let key = read_private(deps.storage, KeyType::Decrypting);

            let msgs : StdResult<Vec<CosmosMsg>, > = msgs
                .iter()
                .map(|msg| 
                    validate_encrypted(deps.as_ref(), msg, &key)
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

        QueryMsg::ContractVersion {} => {
            to_binary(&ContractVersion {
                contract: CONTRACT_NAME.to_string(),
                version: CONTRACT_VERSION.to_string(),
            })
        },

        QueryMsg::SupportedInterface { name, version } => {

            let supported = name == CONTRACT_NAME &&
                if version.is_some() {
                    version.as_ref().unwrap() == CONTRACT_VERSION
                } else {
                    true
                };
            to_binary(&supported)
        }

        QueryMsg::SupportedInterfaces { interfaces } => {

            let supported : Vec<bool> = interfaces
                .iter()
                .map(|(name, version)| 
                    name == CONTRACT_NAME &&
                    if version.is_some() {
                        version.as_ref().unwrap() == CONTRACT_VERSION
                    } else {
                        true
                    }
                )
                .collect();

            to_binary(&supported)
        },


        QueryMsg::CanExecute { msg, .. } => {

            let key = read_private(deps.storage, KeyType::Decrypting);

            let can_execute = 
                if let Ok(_) = validate_encrypted(deps, &msg, &key) {
                    true
                } else {
                    false
                };

            to_binary(&CanExecuteResponse { can_execute  })
        },

        QueryMsg::ValidSignature { signature, data, .. } => {
            let signed: Binary = sign(deps, &data.as_slice())?;
            to_binary(&ValidSignatureResponse {
                is_valid: signed.as_slice() == signature,
            })
        },

        QueryMsg::ValidSignatures { signatures, data, .. } => {
  
            let are_valid : Vec<bool> = signatures
                .iter()
                .enumerate()
                .map(|(i, signature)| {
                    let signed: Binary = sign(deps, &data.get(i).unwrap()).unwrap();
                    signed.as_slice() == signature.as_slice()
                })
                .collect();

            to_binary(&ValidSignaturesResponse {
                are_valid,
            })
        }

        QueryMsg::Signature { to_sign } => sign(deps, &to_sign),
    }
}


fn sign(deps: Deps, to_sign: &[u8]) -> StdResult<Binary> {
    let key = read_private(deps.storage, KeyType::Signing);
    let key = SigningKey::from_slice(&key).unwrap();
    let hash = Sha256::new().chain(to_sign);
    let signature: Signature = key.sign_digest(hash);
    let signature : Binary = signature.to_vec().into();
    Ok(signature)
}

fn decrypt(
    deps: Deps, 
    to_decrypt: &[u8],
    key: &[u8]
) -> StdResult<Binary> {
    let key = SecretKey::parse_slice(key).unwrap();

    let decrypted : Binary = sym_decrypt(
        &key.serialize(), 
        to_decrypt
    ).unwrap().into();

    Ok(decrypted)
}

fn validate_encrypted(
    deps: Deps,
    msg: &CosmosMsg<EncryptedMsg>,
    key: &[u8]
) -> StdResult<CosmosMsg> {

    match msg {
        CosmosMsg::Custom(msg) => {
            let decrypted = decrypt(deps, &msg.encrypted_msg, key)?;
            Ok(from_binary(&decrypted)?)
        },
        _ => Err(cosmwasm_std::StdError::GenericErr { 
            msg: "Only EnctyptedMsg is supported".into() 
        })
    }

    

}