use types::wasm::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary,
};
use cw81::{ValidSignatureResponse, ValidSignaturesResponse};

use crate::{msg::{QueryMsg, InstantiateMsg}, state::PUBKEY};

pub const CONTRACT_NAME: &str = "crates:cw81-pubkey";
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
        &[cw22::ContractSupportedInterface {
            supported_interface: cw81::INTERFACE_NAME.into(),
            version: CONTRACT_VERSION.into()
        }]
    )?;
    PUBKEY.save(deps.storage, &msg.pubkey)?;
    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PubKey {} => to_json_binary(&PUBKEY.load(deps.storage)?),

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

