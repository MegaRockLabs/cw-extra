use cosmwasm_std::from_binary;
use test_context::test_context;
use cw83_tba_registry::msg::{self as RegistryMsg, TokenInfo};
use RegistryMsg::QueryMsg as RegistryQuery;

use crate::helpers::{
    chain::Chain, helper::{full_setup, wasm_query},
};


#[test_context(Chain)]
#[test]
#[ignore]
fn test_queries(chain: &mut Chain) {
    let data = full_setup(chain).unwrap();

    let res = wasm_query(chain, &data.registry, &RegistryQuery::Accounts { 
        skip: None, 
        limit: None
    }).unwrap();
    let accounts = from_binary::<RegistryMsg::AccountsResponse>(
        &res.res.data.unwrap().into()
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
    let col_accounts = from_binary::<RegistryMsg::CollectionAccountsResponse>(
        &res.unwrap().res.data.unwrap().into()
    ).unwrap();



    // 1 account should be registered
    assert_eq!(accounts.len(), 1);
    assert_eq!(col_accounts.len(), 1);

    let first_account = accounts[0].clone();
    let firt_col_account = col_accounts[0].clone();
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

    let info = from_binary::<RegistryMsg::AccountInfoResponse>(
        &res.res.data.unwrap().into()
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

    let res = from_binary::<RegistryMsg::CollectionsResponse>(
        &res.unwrap().res.data.unwrap().into()
    ).unwrap();

    assert_eq!(res.collections.len(), 1);
    let first = res.collections[0].clone();
    assert_eq!(first, data.collection);


}
