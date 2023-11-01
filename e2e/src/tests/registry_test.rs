use cosmrs::crypto::secp256k1;
use cosmwasm_std::Binary;
/* use assert_matches::assert_matches;
use cosm_orc::orchestrator::cosm_orc::tokio_block;
use cosm_orc::orchestrator::error::CosmwasmError::TxError;
use cosm_orc::orchestrator::error::ProcessError;
use cosm_orc::orchestrator::Coin as OrcCoin;
use cosm_orc::orchestrator::ExecReq;
use std::collections::HashMap;
use std::env;
use std::time::Duration; */
use test_context::test_context;

use crate::helpers::{
    chain::Chain,
    helper::{
        instantiate_registry, latest_block_time, instantiate_collection, instantiate_proxy, mint_token, create_token_account, query_token_owner /* ,  gen_users, CREATION_FEE,
        MAX_TOKENS, MINT_PRICE, */
    },
};


#[test_context(Chain)]
#[test]
#[ignore]
fn test_instantiate_registry(chain: &mut Chain) {
    let user = chain.cfg.users[0].clone();
    instantiate_registry(chain, user.account.address, &user.key).unwrap();
}



#[test_context(Chain)]
#[test]
#[ignore]
fn test_instantiate_proxy(chain: &mut Chain) {
    let user = chain.cfg.users[0].clone();
    instantiate_proxy(chain, user.account.address, &user.key).unwrap();
}



#[test_context(Chain)]
#[test]
#[ignore]
fn test_instantiate_collection(chain: &mut Chain) {
    let user = chain.cfg.users[0].clone();

    /* let infos: Vec<(String, DeployInfo)> =  chain.cfg.orc_cfg.contract_deploy_info.clone().into_iter().collect();
    println!("infos: {:?}", infos);
    */

    let res = instantiate_proxy(chain, user.account.address.clone(), &user.key).unwrap().address;
    instantiate_collection(
        chain, 
        user.account.address,
        res.to_string(),
        &user.key
    ).unwrap();

}

#[test_context(Chain)]
#[test]
#[ignore]
fn test_mint_token(chain: &mut Chain) {
    let user = chain.cfg.users[0].clone();
    let proxy = instantiate_proxy(chain, user.account.address.clone(), &user.key).unwrap().address;
   
    let init_res = instantiate_collection(
        chain, 
        user.account.address.clone(), 
        proxy.clone().to_string(),
        &user.key
    ).unwrap();

    let tags = init_res
                        .res
                        .find_event_tags(
                            "instantiate".to_string(), 
                            "_contract_address".to_string()
                        );

    let col_address = tags[0].value.clone();
    
    let mint_res = mint_token(
        chain, 
        col_address.clone(), 
        user.account.address.clone(), 
        &user.key
    ).unwrap();


    let token_id = mint_res
                .res
                .find_event_tags(
                    "wasm".to_string(), 
                    "token_id".to_string()
                )[0].value.clone();


    let owner_res = query_token_owner(
        chain,
        col_address,
        token_id,
    ).unwrap();

    assert_eq!(user.account.address, owner_res.owner)

}



#[test_context(Chain)]
#[test]
#[ignore]
fn test_create_token_account(chain: &mut Chain) {

    let user = chain.cfg.users[0].clone();
    let user_addr = &user.account.address;
    

    instantiate_registry(chain, user_addr.to_string(), &user.key).unwrap();

    let _start_time = latest_block_time(chain).plus_seconds(60);
    
    let proxy = instantiate_proxy(chain, user.account.address.clone(), &user.key).unwrap().address;
   
    let init_res = instantiate_collection(
        chain, 
        user.account.address.clone(), 
        proxy.clone().to_string(),
        &user.key
    ).unwrap();

    let col_address  = init_res
                        .res
                        .find_event_tags(
                            "instantiate".to_string(), 
                            "_contract_address".to_string()
                        )[0].value.clone();

    
    let mint_res = mint_token(
        chain, 
        col_address.clone(), 
        user.account.address.clone(), 
        &user.key
    ).unwrap();


    let token_id = mint_res
                .res
                .find_event_tags(
                    "wasm".to_string(), 
                    "token_id".to_string()
                )[0].value.clone();


    let signing : secp256k1::SigningKey = user.key.clone().try_into().unwrap();
    let pubkey : Binary = signing.public_key().to_bytes().into();
    


    let create_res = create_token_account(
        chain, 
        col_address,
        token_id,
        user_addr.clone(),
        pubkey,
        &user.key
    ).unwrap();


    let aaa = create_res
            .res
            .find_event_tags(
                "reply".into(),
                "_contract_address".into()
            )[0].value.clone();
    

    assert!(aaa.len() > 0);

}
