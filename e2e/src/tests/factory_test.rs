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
fn test_instantiate_factory(chain: &mut Chain) {
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


/*
#[test_context(Chain)]
#[test]
#[ignore]
fn test_start_trading_time(chain: &mut Chain) {
    let denom = chain.cfg.orc_cfg.chain_cfg.denom.clone();
    let user = chain.cfg.users[0].clone();
    let user_addr = &user.account.address;

    let initial_total_supply =
        tokio_block(chain.orc.client.bank_query_supply(denom.parse().unwrap()))
            .unwrap()
            .balance;

    instantiate_factory(chain, user_addr.to_string(), &user.key).unwrap();

    let start_time = latest_block_time(chain).plus_seconds(5);

    let minter_msg = Sg2ExecuteMsg::CreateMinter(create_minter_msg(
        chain,
        user_addr.to_string(),
        1000,
        10,
        start_time,
        Some(start_time.plus_seconds(60 * 60)),
    ));

    let res = chain
        .orc
        .execute(
            FACTORY_NAME,
            "factory_exec_minter_inst_w_trading_time",
            &minter_msg,
            &user.key,
            vec![OrcCoin {
                amount: CREATION_FEE,
                denom: denom.parse().unwrap(),
            }],
        )
        .unwrap();

    let tags = res
        .res
        .find_event_tags("instantiate".to_string(), "_contract_address".to_string());

    let (minter_addr, sg721_addr) = (tags[0].value.clone(), tags[1].value.clone());
    assert!(!minter_addr.trim().is_empty());
    assert!(!sg721_addr.trim().is_empty());

    let mut total_mints = 0;
    let mut total_fairburn_fees = 0;

    let fair_burn_fees = res
        .res
        .find_event_tags("fund_fairburn_pool".to_string(), "amount".to_string());

    let amount = fair_burn_fees[0].value.split(&denom).collect::<Vec<&str>>()[0];
    total_fairburn_fees += amount.parse::<u128>().unwrap();

    let users = gen_users(chain, 20, MINT_PRICE * 12, None);
    let num_users = users.len() as u32;

    chain
        .orc
        .contract_map
        .add_address("minter", minter_addr)
        .unwrap();

    // Sleep to ensure we can start minting
    chain
        .orc
        .poll_for_n_secs(6, Duration::from_millis(20_000))
        .unwrap();

    let init_balance = tokio_block(
        chain
            .orc
            .client
            .bank_query_balance(user_addr.parse().unwrap(), denom.parse().unwrap()),
    )
    .unwrap()
    .balance;

    for user in &users {
        let mut reqs = vec![];
        for _ in 0..100 / num_users {
            total_mints += 1;
            reqs.push(ExecReq {
                contract_name: "minter".to_string(),
                msg: Box::new(vending_minter::msg::ExecuteMsg::Mint {}),
                funds: vec![OrcCoin {
                    amount: MINT_PRICE,
                    denom: denom.parse().unwrap(),
                }],
            });
        }

        let res = chain
            .orc
            .execute_batch("minter_batch_exec_mint_token_w_trading_time", reqs, user)
            .unwrap();

        let fair_burn_fees = res
            .res
            .find_event_tags("fund_fairburn_pool".to_string(), "amount".to_string());

        for fee in fair_burn_fees {
            let amount = fee.value.split(&denom).collect::<Vec<&str>>()[0];
            total_fairburn_fees += amount.parse::<u128>().unwrap();
        }
    }

    assert_eq!(total_mints, 100);

    let balance = tokio_block(
        chain
            .orc
            .client
            .bank_query_balance(user_addr.parse().unwrap(), denom.parse().unwrap()),
    )
    .unwrap()
    .balance;

    // 100 x MINT_PRICE = 10k STARS x 0.9 (10% fee)
    assert_eq!(balance.amount, init_balance.amount + 9_000_000_000);

    // fairburn fees
    // half of the 10% fees should be sent to fairburn pool
    // 500STARS + 500STARS initially sent for collection creation fee
    assert_eq!(total_fairburn_fees, 1_000_000_000);

    let total_supply = tokio_block(chain.orc.client.bank_query_supply(denom.parse().unwrap()))
        .unwrap()
        .balance;

    // the other half burned
    assert_eq!(
        initial_total_supply.amount - 1_000_000_000,
        total_supply.amount
    );
}

#[test_context(Chain)]
#[test]
#[ignore]
fn test_invalid_start_trading_time(chain: &mut Chain) {
    let denom = chain.cfg.orc_cfg.chain_cfg.denom.clone();
    let user = chain.cfg.users[0].clone();
    let user_addr = &user.account.address;

    instantiate_factory(chain, user_addr.to_string(), &user.key).unwrap();

    let start_time = latest_block_time(chain).plus_seconds(100_000);

    let minter_msg = Sg2ExecuteMsg::CreateMinter(create_minter_msg(
        chain,
        user_addr.to_string(),
        1000,
        10,
        start_time,
        Some(start_time.plus_seconds(60 * 60 * 24 * 365)),
    ));

    let res = chain.orc.execute(
        FACTORY_NAME,
        "factory_exec_minter_inst_w_trading_time_err",
        &minter_msg,
        &user.key,
        vec![OrcCoin {
            amount: CREATION_FEE,
            denom: denom.parse().unwrap(),
        }],
    );

    let err = res.unwrap_err();
    assert_matches!(err, ProcessError::CosmwasmError(TxError(..)));
    assert!(err.to_string().contains("InvalidStartTradingTime"));
}
 */