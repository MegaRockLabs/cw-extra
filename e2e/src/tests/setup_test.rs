use test_context::test_context;

use crate::helpers::{
    chain::Chain,
    helper::{
        instantiate_registry, instantiate_collection, instantiate_proxy, 
        mint_token, query_token_owner, full_setup
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
        "1".to_string(),
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

    let data = full_setup(chain).unwrap();
    assert!(data.token_account.len() > 0);

}
