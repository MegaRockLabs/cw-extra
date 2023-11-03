use cosmwasm_std::{StdResult, Deps, Order};

use crate::{state::TOKEN_TO_CONTRACT, msg::{AccountInfoResponse, TokenInfo, CollectionAccount, CollectionAccountsResponse}};

const DEFAULT_BATCH_SIZE : u32 = 100;


pub fn account_info(
    deps: Deps,
    info: TokenInfo
) -> StdResult<AccountInfoResponse> {
            
    let address = TOKEN_TO_CONTRACT.load(
        deps.storage, 
        (info.contract.as_str(), info.id.as_str())
    )?;

    Ok(AccountInfoResponse {
        address, info: None
    })
}

pub fn collection_accounts(
    deps: Deps,
    collection: &str,
    start_after: Option<u32>,
    limit: Option<u32>
) -> StdResult<CollectionAccountsResponse> {
            
    TOKEN_TO_CONTRACT
        .prefix(collection)
        .range(
            deps.storage, 
            None, 
            None, 
            Order::Ascending
        )
        .enumerate()
        .filter(|(i, _)| {
            if let Some(start_after) = start_after {
                i >= &(start_after as usize)
            } else {
                true
            }
        })
        .take(limit.unwrap_or(DEFAULT_BATCH_SIZE) as usize)
        .map(|(_,item)| {
            let (id, address) = item?;
            Ok(CollectionAccount { id, address })
        })
        .collect()

}