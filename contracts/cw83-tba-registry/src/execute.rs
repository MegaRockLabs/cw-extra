use cosmwasm_std::{Response, Env, Binary, DepsMut, Coin};
use cw83::Cw83RegistryContract;

use crate::{
    state::{LAST_ATTEMPTING, StoredAccount},
    helpers::verify_nft_ownership, 
    error::ContractError
};

pub fn create_account(
    deps: DepsMut,
    env: Env,
    sender: String,
    code_id: u64,
    init_msg: Binary,
    token_contract: String,
    token_id : String,
    funds: Vec<Coin>
) -> Result<Response, ContractError> {
    verify_nft_ownership(deps.as_ref(), &sender, token_contract.clone(), token_id.clone())?;

    let contract = Cw83RegistryContract(env.contract.address);

    LAST_ATTEMPTING.save(deps.storage, &StoredAccount{
        owner: sender,
        token_contract: token_contract.clone(),
        token_id: token_id.clone()
    })?;

    Ok(
        Response::default()
        .add_submessage(contract.create_account_submsg(
            code_id, 
            init_msg, 
            token_contract, 
            token_id, 
            funds
        )?)
    )
}