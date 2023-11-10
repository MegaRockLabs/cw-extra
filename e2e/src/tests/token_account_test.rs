use cosmwasm_std::{Binary, CosmosMsg, BankMsg, WasmMsg, from_binary, Empty};
use cw_ownable::Ownership;
use test_context::test_context;

use cw82_token_account::msg::{QueryMsg, AssetsResponse, Status, TokenInfo};

use crate::helpers::{
    chain::Chain, helper::{full_setup, can_execute, wasm_query},
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
        CosmosMsg::Stargate { 
            type_url: String::default(), 
            value: Binary::default() 
        }
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
        &QueryMsg::<Empty>::Pubkey {}
    );
    let pubkey = from_binary::<Binary>(
        &res.unwrap().res.data.unwrap().into()
    ).unwrap();

    assert_eq!(pubkey, data.public_key);



    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::<Empty>::Status {} 
    );
    let status = from_binary::<Status>(
        &res.unwrap().res.data.unwrap().into()
    ).unwrap();

    assert_eq!(status, Status { frozen: false });


    

    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::<Empty>::Ownership {} 
    );
    let ownership = from_binary::<Ownership<String>>(
        &res.unwrap().res.data.unwrap().into()
    ).unwrap();

    assert_eq!(ownership.owner, Some(data.user_address));




    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::<Empty>::Token {} 
    );
    let info = from_binary::<TokenInfo>(
        &res.unwrap().res.data.unwrap().into()
    ).unwrap();

    assert_eq!(info, TokenInfo {
        token_contract: data.collection,
        token_id: data.token_id
    });


    


    let res = wasm_query(
        chain, 
        &data.token_account, 
        &QueryMsg::<Empty>::Assets { skip: None, limit: None }
    );
    let assets = from_binary::<AssetsResponse>(
        &res.unwrap().res.data.unwrap().into()
    ).unwrap();

    assert_eq!(assets.balances, vec![]);




}
