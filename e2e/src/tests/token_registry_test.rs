use cosmwasm_std::from_json;
use test_context::test_context;
use cw83_tba_registry::msg::{self as RegistryMsg, TokenInfo};
use RegistryMsg::QueryMsg as RegistryQuery;

use crate::helpers::{
    chain::Chain, helper::{full_setup, wasm_query, mint_token},
};


#[test_context(Chain)]
#[test]
#[ignore]
fn basic_queries(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();

    let res = wasm_query(chain, &data.registry, &RegistryQuery::Accounts { 
        skip: None, 
        limit: None
    }).unwrap();

    let acc_res = from_json::<RegistryMsg::AccountsResponse>(
        &res.res.data.unwrap()
    ).unwrap();


    let res = wasm_query(
        chain, 
        &data.registry, 
        &RegistryQuery::CollectionAccounts { 
            collection: data.collection.clone(), 
            skip: None, 
            limit: None 
        }
    );
    let col_res = from_json::<RegistryMsg::CollectionAccountsResponse>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();


    // 1 account should be registered
    assert_eq!(acc_res.total, 1);
    assert_eq!(acc_res.accounts.len(), 1);
    assert_eq!(col_res.total, 1);
    assert_eq!(col_res.accounts.len(), 1);

    let first_account = acc_res.accounts.first().clone().unwrap();
    let firt_col_account = col_res.accounts.first().clone().unwrap();
    assert_eq!(first_account.address, firt_col_account.address);
    assert_eq!(first_account.id, firt_col_account.id);


    let res = wasm_query(
        chain, 
        &data.registry, 
        &RegistryQuery::AccountInfo(
            RegistryMsg::AccountQuery {
                query: TokenInfo {
                    collection: data.collection.clone(),
                    id: data.token_id.clone(),
                }
            }
        )
    ).unwrap();

    let info = from_json::<RegistryMsg::AccountInfoResponse>(
        &res.res.data.unwrap()
    ).unwrap();

    assert_eq!(info.address, data.token_account);
    assert_eq!(info.address, first_account.address);

    
    let res = wasm_query(
        chain, 
        &data.registry, 
        &RegistryQuery::Collections  { 
            skip: None, 
            limit: None 
        }
    );

    let res = from_json::<RegistryMsg::CollectionsResponse>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();

    assert_eq!(res.collections.len(), 1);
    let first = res.collections[0].clone();
    assert_eq!(first, data.collection);


}




#[test_context(Chain)]
#[test]
#[ignore]
fn account_skip_limits(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();
    let user = chain.cfg.users[0].clone();


    for id in ["2", "3", "4", "5", "6"].into_iter() {
        mint_token(
            chain, 
            data.collection.clone(), 
            id.to_string(),
            data.token_account.clone(), 
            &user.key
        ).unwrap();
    }

    let res = wasm_query(chain, &data.registry, &RegistryQuery::Accounts { 
        skip: None, 
        limit: None
    }).unwrap();

    let acc_res = from_json::<RegistryMsg::AccountsResponse>(
        &res.res.data.unwrap()
    ).unwrap();


    let res = wasm_query(
        chain, 
        &data.registry, 
        &RegistryQuery::CollectionAccounts { 
            collection: data.collection.clone(), 
            skip: None, 
            limit: None 
        }
    );
    let col_res = from_json::<RegistryMsg::CollectionAccountsResponse>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();


    // 1 account should be registered
    assert_eq!(acc_res.total, 1);
    assert_eq!(acc_res.accounts.len(), 1);
    assert_eq!(col_res.total, 1);
    assert_eq!(col_res.accounts.len(), 1);

    let first_account = acc_res.accounts.first().clone().unwrap();
    let firt_col_account = col_res.accounts.first().clone().unwrap();
    assert_eq!(first_account.address, firt_col_account.address);
    assert_eq!(first_account.id, firt_col_account.id);


    let res = wasm_query(
        chain, 
        &data.registry, 
        &RegistryQuery::AccountInfo(
            RegistryMsg::AccountQuery {
                query: TokenInfo {
                    collection: data.collection.clone(),
                    id: data.token_id.clone(),
                }
            }
        )
    ).unwrap();

    let info = from_json::<RegistryMsg::AccountInfoResponse>(
        &res.res.data.unwrap()
    ).unwrap();

    assert_eq!(info.address, data.token_account);
    assert_eq!(info.address, first_account.address);

    
    let res = wasm_query(
        chain, 
        &data.registry, 
        &RegistryQuery::Collections  { 
            skip: None, 
            limit: None 
        }
    );

    let res = from_json::<RegistryMsg::CollectionsResponse>(
        &res.unwrap().res.data.unwrap()
    ).unwrap();

    assert_eq!(res.collections.len(), 1);
    let first = res.collections[0].clone();
    assert_eq!(first, data.collection);


}
