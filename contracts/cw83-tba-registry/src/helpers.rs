use cosmwasm_std::Deps;

use crate::{error::ContractError, msg::TokenInfo};


pub fn construct_label(
    info: &TokenInfo
) -> String {
    format!("{}-{}-account", info.collection, info.id)
}


pub fn verify_nft_ownership(
    deps: Deps,
    sender: &str,
    token_info: TokenInfo
) -> Result<(), ContractError> {

    let owner_res = deps
        .querier
        .query_wasm_smart::<cw721::OwnerOfResponse>(
            token_info.collection, 
        &sg721_base::QueryMsg::OwnerOf {
            token_id: token_info.id,
            include_expired: None
        }
    )?;

    if owner_res.owner.as_str() != sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

