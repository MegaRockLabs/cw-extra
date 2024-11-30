use cosm_orc::orchestrator::cosm_orc::tokio_block;
use cosm_tome::{modules::bank::model::SendRequest, chain::request::TxOptions};
use cosm_tome::chain::coin::Coin;
use cosmwasm_std::{from_json, AnyMsg, BankMsg, Binary, CosmosMsg, WasmMsg};
use cw_ownable::Ownership;
use test_context::test_context;

use cw82_token_account::msg::{
    QueryMsg, 
    AssetsResponse, 
    Status, 
    TokenInfo, 
    KnownTokensResponse,
    ExecuteMsg as TAExecuteMsg
};


use crate::helpers::helper::{mint_token, send_token, ACOUNT_NAME, create_token_account, get_init_address};
use crate::helpers::{
    chain::Chain, helper::{full_setup, can_execute, wasm_query, wasm_query_typed},
};



#[test_context(Chain)]
#[test]
#[ignore]
fn can_execute_test(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();

    
    assert!(can_execute(
        chain, 
        &data.token_account, 
        data.user_address.clone(), 
        BankMsg::Send {
            to_address: data.user_address.clone(),
            amount: vec![],
        }.into()
    ).can_execute);


    assert!(!can_execute(
        chain, 
        &data.token_account, 
        String::from("not owner"), 
        BankMsg::Send {
            to_address: data.user_address.clone(),
            amount: vec![],
        }.into()
    ).can_execute);


    // general wasm
    assert!(!can_execute(
        chain, 
        &data.token_account, 
        data.user_address.clone(), 
        WasmMsg::Execute { 
            contract_addr: String::from("any"), 
            msg: Binary::from(b"any"), 
            funds: vec![] 
        }.into()
    ).can_execute);


    // stargate
    assert!(!can_execute(
        chain, 
        &data.token_account, 
        data.user_address.clone(), 
        CosmosMsg::Any( AnyMsg { 
            type_url: String::default(), 
            value: Binary::default() 
        })
    ).can_execute);



}


#[test_context(Chain)]
#[test]
#[ignore]
fn general_queries(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();


    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::Pubkey {}
    );
    let pubkey = from_json::<Binary>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();

    assert_eq!(pubkey, data.public_key);



    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::Status {} 
    );
    let status = from_json::<Status>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();

    assert_eq!(status, Status { frozen: false });


    

    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::Ownership {} 
    );
    let ownership = from_json::<Ownership<String>>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();

    assert_eq!(ownership.owner, Some(data.user_address));




    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::Token {} 
    );
    let info = from_json::<TokenInfo>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();

    assert_eq!(info, TokenInfo {
        token_contract: data.collection,
        token_id: data.token_id
    });


    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::Assets { skip: None, limit: None }
    );
    let assets = from_json::<AssetsResponse>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();

    assert_eq!(assets.balances, vec![]);




}



#[test_context(Chain)]
#[test]
#[ignore]
fn known_assets(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();
    let user = chain.cfg.users[0].clone();
    

    let assets : AssetsResponse = wasm_query_typed(
        chain, 
        &data.token_account, 
        &QueryMsg::Assets { skip: None, limit: None }
    ).unwrap();

    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account, 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();

    assert_eq!(assets.balances, vec![]);
    assert_eq!(assets.tokens, vec![]);
    assert_eq!(tokens.len(), 0);


    let denom = chain.cfg.orc_cfg.chain_cfg.denom.clone();

    tokio_block(async { 
        chain.orc.client.bank_send(
            SendRequest {
                amounts: vec![Coin {
                    denom: denom.clone().parse().unwrap(),
                    amount: 1000u128
                }], 
                from: user.account.address.parse().unwrap(),
                to: data.token_account.parse().unwrap() 
            }, 
            &user.key, 
            &TxOptions { 
                timeout_height: None, 
                fee: None, 
                memo: String::default()
            }
        )
        .await 
    }).unwrap();


    let assets : AssetsResponse = wasm_query_typed(
        chain, 
        &data.token_account, 
        &QueryMsg::Assets { skip: None, limit: None }
    ).unwrap();


    assert_eq!(assets.balances, vec![cosmwasm_std::Coin {
        denom: denom.clone(),
        amount: 1000u128.into()
    }]);


}


#[test_context(Chain)]
#[test]
#[ignore]
fn know_tokens_on_recieve(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();
    let user = chain.cfg.users[0].clone();
    
    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account, 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 0);


    let mint_res = mint_token(
        chain, 
        data.collection.clone(), 
        "2".into(), 
        user.account.address.clone(), 
        &user.key
    ).unwrap();


    let token_id = mint_res
                .res
                .find_event_tags(
                    "wasm".to_string(), 
                    "token_id".to_string()
                )[0].value.clone();


    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account, 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();

    assert_eq!(tokens.len(), 0);


    send_token(
        chain, 
        token_id.clone(), 
        data.token_account.clone(),
        Binary::default(),
        &user.key
    ).unwrap();


    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();

    assert_eq!(tokens.len(), 1);

    let first = tokens.first().unwrap().clone();

    assert_eq!(first, TokenInfo { token_contract: data.collection, token_id: token_id })
}



#[test_context(Chain)]
#[test]
#[ignore]
fn tokens_receving(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();
    let user = chain.cfg.users[0].clone();
    
    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account, 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 0);

    let token_id = "2".to_string();

    // mint direclty to the token account
    mint_token(
        chain, 
        data.collection.clone(), 
        token_id.clone(),
        data.token_account.clone(), 
        &user.key
    ).unwrap();


    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account, 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    
    // account does not know about the token 
    assert_eq!(tokens.len(), 0);


    chain.orc.execute(
        ACOUNT_NAME,
        "acc_tokens_ack",
        &TAExecuteMsg::UpdateKnownTokens { 
            collection: data.collection.clone(), 
            start_after: None, 
            limit: None 
        },
        &user.key,
        vec![],
    ).unwrap();


    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();

    assert_eq!(tokens.len(), 1);
}



#[test_context(Chain)]
#[test]
#[ignore]
fn tokens_acknowlegement(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();
    let user = chain.cfg.users[0].clone();
    
    
    // minting 3 token for token account
    for id in ["2", "3", "4", "5", "6"].into_iter() {
        mint_token(
            chain, 
            data.collection.clone(), 
            id.to_string(),
            data.token_account.clone(), 
            &user.key
        ).unwrap();
    }

    // making the account aware of the tokens it owns
    chain.orc.execute(
        ACOUNT_NAME,
        "acc_tokens_ack",
        &TAExecuteMsg::UpdateKnownTokens { 
            collection: data.collection.clone(), 
            start_after: None, 
            limit: None 
        },
        &user.key,
        vec![],
    ).unwrap();

    // ----------------------------------------------------

    // Transfering to EOA after which there are only 2 left
    chain.orc.execute(
        ACOUNT_NAME,
        "acc_token_acc",
        &TAExecuteMsg::TransferToken { 
            collection: data.collection.clone(), 
            token_id: "2".into(), 
            recipient: user.account.address.clone(), 
        },
        &user.key,
        vec![],
    ).unwrap();

    
    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 4);


    // ----------------------------------------------------


    // sending to itself should be fine but not change anything
    chain.orc.execute(
        ACOUNT_NAME,
        "acc_token_acc",
        &TAExecuteMsg::SendToken { 
            collection: data.collection.clone(), 
            token_id: "3".into(), 
            contract: data.token_account.clone(), 
            msg: Binary::default() 
        },
        &user.key,
        vec![],
    ).unwrap();

    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 4);


    // ----------------------------------------------------

    // create account with newly received token 2
    let create_res = create_token_account(
        chain, 
        data.collection.clone(),
        "2".to_string(),
        data.public_key,
        &user.key
    ).unwrap();

    let second_ta = get_init_address(create_res.res);


    // sending to an nft to second token account
    chain.orc.execute(
        ACOUNT_NAME,
        "acc_token_acc",
        &TAExecuteMsg::SendToken { 
            collection: data.collection.clone(), 
            token_id: "3".into(), 
            contract: second_ta.clone(), 
            msg: Binary::default() 
        },
        &user.key,
        vec![],
    ).unwrap();


    // first token account now only knows about 3 tokens
    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.first().unwrap().token_id, "4".to_string());



    // second token account only knows about 1 token "3"
    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &second_ta.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens.first().unwrap().token_id, "3".to_string());


    // ----------------------------------------------------

    // making the token account forget about the tokens it owns
    chain.orc.execute(
        ACOUNT_NAME,
        "acc_token_acc",
        &TAExecuteMsg::ForgetTokens { 
            collection: data.collection.clone(), 
            token_ids: vec!["4".to_string()] 
        },
        &user.key,
        vec![],
    ).unwrap();

    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 2);


    // forget about the whole collection
    chain.orc.execute(
        ACOUNT_NAME,
        "acc_token_acc",
        &TAExecuteMsg::ForgetTokens { 
            collection: data.collection.clone(), 
            token_ids: vec![] 
        },
        &user.key,
        vec![],
    ).unwrap();

    let tokens : KnownTokensResponse = wasm_query_typed(
        chain, 
        &data.token_account.clone(), 
        &QueryMsg::KnownTokens { skip: None, limit: None }
    ).unwrap();
    assert_eq!(tokens.len(), 0);



    // ----------------------------------------------------


    // Token Account balance is still ok (just unaware of it)
    let res : cw721::TokensResponse = wasm_query_typed(
        chain, 
        &data.collection, 
        &sg721_base::QueryMsg::Tokens { 
            owner: data.token_account.clone(),
            start_after: None, 
            limit: None 
        }
    ).unwrap();
    assert_eq!(res.tokens.len(), 3);
    assert_eq!(res.tokens, vec![
        String::from("4"),
        String::from("5"),
        String::from("6"),
    ]);

    // Other balances ok
    let res : cw721::TokensResponse = wasm_query_typed(
        chain, 
        &data.collection, 
        &sg721_base::QueryMsg::Tokens { 
            owner: second_ta.clone(),
            start_after: None, 
            limit: None 
        }
    ).unwrap();
    assert_eq!(res.tokens.len(), 1);
    assert_eq!(res.tokens.first().unwrap(), &String::from("3"));


    let res : cw721::TokensResponse = wasm_query_typed(
        chain, 
        &data.collection, 
        &sg721_base::QueryMsg::Tokens { 
            owner: user.account.address.clone(),
            start_after: None, 
            limit: None 
        }
    ).unwrap();
    assert_eq!(res.tokens.len(), 2);
    assert_eq!(res.tokens.first().unwrap(), &String::from("1"));
    assert_eq!(res.tokens.last().unwrap(), &String::from("2"));
}