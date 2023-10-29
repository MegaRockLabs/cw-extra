use cosmwasm_std::{Deps, to_binary, Binary, from_binary};
use cw721::{Cw721QueryMsg, OwnerOfResponse};

use crate::error::ContractError;



pub fn verify_nft_ownership(
    deps: Deps,
    sender: &String,
    token_contract: String,
    token_id : String,
) -> Result<(), ContractError> {

    let binary_res : Binary = deps.querier.query_wasm_smart(
        token_contract, 
        &to_binary(&Cw721QueryMsg::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None
        })?
    )?;

    let res : OwnerOfResponse = from_binary(&binary_res)?;
    if res.owner == *sender {
        return Ok(());
    } else {
        return Err(ContractError::Unauthorized {});
    }
}