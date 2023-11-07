
use cosmwasm_std::{entry_point, Response, DepsMut, MessageInfo, Env, StdResult, Binary, Deps, to_binary};
use crate::{msg::{QueryMsg, InstantiateMsg, ContractError, ValidSignatureResponse, ValidSignaturesResponse}, state::{save_private, read_private}};


use k256::ecdsa::{
        signature::DigestSigner,
        SigningKey, Signature
    };

use rand_chacha::{
    ChaChaRng, 
    rand_core::SeedableRng
};


pub const CONTRACT_NAME: &str = "crates:cw81-sn-ks";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


use sha2::{
    Sha256, 
    digest::{Update, Digest}
};

#[entry_point]
pub fn instantiate(deps: DepsMut, env : Env, _ : MessageInfo, _ : InstantiateMsg,) 
-> Result<Response, ContractError> {   
    let mut ring = ChaChaRng::from_seed(env.block.random.unwrap().to_array()?);
    let secret_key = SigningKey::random(&mut ring);
    let secret_binary : Binary = secret_key.to_bytes().as_slice().into();
    save_private(deps.storage, &secret_binary)?;
    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {

        QueryMsg::Signature { to_sign } => to_binary(&sign(deps, &to_sign)?),

        QueryMsg::ValidSignature { signature, data, .. } => {
            let signed: Binary = sign(deps, &data)?;
            to_binary(&ValidSignatureResponse {
                is_valid: signed == signature,
            })
        },

        QueryMsg::ValidSignatures { signatures, data, .. } => {
  
            let are_valid : Vec<bool> = signatures
                .iter()
                .enumerate()
                .map(|(i, signature)| {
                    let signed: Binary = sign(
                        deps, 
                        data.get(i).unwrap()
                    ).unwrap();
                    signed == *signature
                })
                .collect();

            to_binary(&ValidSignaturesResponse {
                are_valid,
            })

        }
    }
}


fn sign(
    deps: Deps,
    to_sign: &Binary
) -> StdResult<Binary> {
    let key = read_private(deps.storage);
    let key = SigningKey::from_slice(&key).unwrap();
    let hash = Sha256::new().chain(to_sign.as_slice());
    let signature: Signature = key.sign_digest(hash);
    let signature : Binary = signature.to_bytes().to_vec().into();
    Ok(signature)
}