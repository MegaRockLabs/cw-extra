use cosmwasm_std::{StdResult, Deps, Binary, from_binary, Order, Env};
use cw82::{CanExecuteResponse, ValidSignatureResponse, ValidSignaturesResponse};
use k256::sha2::{Digest, Sha256};

use crate::{
    state::{OWNER, PUBKEY, KNOWN_TOKENS}, 
    utils::{generate_amino_transaction_string, parse_payload}, msg::{AssetsResponse, TokenInfo}
};


pub fn can_execute(
    deps: Deps,
    sender: String
) -> StdResult<CanExecuteResponse> {
    let owner : String = OWNER.load(deps.storage)?;
    Ok(CanExecuteResponse { can_execute: owner == sender })
}


pub fn valid_signature(
    deps: Deps,
    data: Binary,
    signature: Binary,
    payload: &Option<Binary>
) -> StdResult<ValidSignatureResponse> {
    let pk: Binary = PUBKEY.load(deps.storage)?;
    let payload = parse_payload(payload)?;
    Ok(ValidSignatureResponse {
        is_valid: verify_arbitrary(
            deps,
            &payload.account,
            data,
            signature,
            &pk
        ).unwrap_or(false)
    })
}


pub fn valid_signatures(
    deps: Deps,
    data: Vec<Binary>,
    signatures: Vec<Binary>,
    payload: &Option<Binary>
) -> StdResult<ValidSignaturesResponse> {
    let pk: Binary = PUBKEY.load(deps.storage)?;
    let payload = parse_payload(payload)?;

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
    Ok(ValidSignaturesResponse {
        are_valid
    })
}


pub fn verify_arbitrary(
    deps: Deps,
    account_addr: &str,
    data: Binary,
    signature: Binary,
    pubkey: &[u8],
) -> StdResult<bool> {

    let digest = Sha256::new_with_prefix(
        generate_amino_transaction_string(
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


pub fn assets(
    deps: Deps,
    env: Env,
) -> StdResult<AssetsResponse> {

    let nfts = known_tokens(deps)?;
    let balance = deps.querier.query_all_balances(env.contract.address)?;


    Ok(AssetsResponse {
        balances: balance,
        tokens: nfts
    })
}


pub fn known_tokens(
    deps: Deps,
) -> StdResult<Vec<TokenInfo>> {

    let tokens : StdResult<Vec<TokenInfo>> = KNOWN_TOKENS
    .keys(
        deps.storage, 
        None, 
        None, 
        Order::Ascending
    )
    .map(|kp| {
        let kp = kp?;
        Ok(TokenInfo { token_contract: kp.0, token_id: kp.1 })
    })
    .collect();

    tokens
}