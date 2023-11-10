use std::str::FromStr;

use super::chain::Chain;
use super::msg::ProxyInstantiateMsg;
use cosm_orc::orchestrator::cosm_orc::tokio_block;
use cosm_orc::orchestrator::error::{ProcessError, CosmwasmError};
use cosm_orc::orchestrator::{Coin as OrcCoin, ExecResponse, Address, ChainTxResponse, QueryResponse};
use cosm_orc::orchestrator::{InstantiateResponse, SigningKey};
use cosm_tome::chain::request::TxOptions;
use cosm_tome::modules::bank::model::SendRequest;
use cosmrs::crypto::secp256k1;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Empty, CosmosMsg, WasmMsg, Binary, to_binary, from_binary};

use cw1::CanExecuteResponse;
use cw82_token_account::msg::QueryMsg;
use cw83_tba_registry::msg::{InstantiateMsg, CreateAccountMsg, TokenInfo};
use serde::Serialize;

// contract names used by cosm-orc to register stored code ids / instantiated addresses:
pub const REGISTRY_NAME     : &str = "cw83_tba_registry";
pub const ACOUNT_NAME       : &str = "cw82_token_account";
pub const PROXY_NAME        : &str = "cw1_whitelist";
pub const COLLECTION_NAME   : &str = "sg721_base";


pub const MAX_TOKENS: u32 = 10_000;
pub const CREATION_FEE: u128 = 1_000_000_000;
pub const MINT_PRICE: u128 = 100_000_000;

pub fn instantiate_registry(
    chain: &mut Chain,
    creator_addr: String,
    key: &SigningKey,
) -> Result<InstantiateResponse, ProcessError> {
    
    let account_id = chain.orc.contract_map.code_id(ACOUNT_NAME)?;

    chain.orc.instantiate(
        REGISTRY_NAME,
        "registry_instantiate",
        &InstantiateMsg {
            allowed_ids: vec![account_id],
            admins: None,
        },
        key,
        Some(creator_addr.parse().unwrap()),
        vec![],
    )
}


pub fn instantiate_proxy(
    chain: &mut Chain,
    admin: String,
    key: &SigningKey,
) -> Result<InstantiateResponse, ProcessError> {

    chain.orc.instantiate(
        PROXY_NAME,
        "proxy_instantiate",
        &ProxyInstantiateMsg {
            admins: vec![admin.clone()],
            mutable: true,
        },
        key,
        Some(admin.parse().unwrap()),
        vec![],
    )
}

pub fn instantiate_collection(
    chain: &mut Chain,
    creator_addr: String,
    minter: String,
    key: &SigningKey,
) -> Result<ExecResponse, ProcessError> {
    // let infos: Vec<(String, DeployInfo)> =  chain.cfg.orc_cfg.contract_deploy_info.clone().into_iter().collect();
    
    let code_id = chain.orc.contract_map.code_id(COLLECTION_NAME)?;

    let init_msg : CosmosMsg::<Empty> = CosmosMsg::Wasm(WasmMsg::Instantiate { 
        admin: Some(minter.clone().to_string()), 
        code_id, 
        msg: to_binary(&sg721::InstantiateMsg {
            name: "test".into(),
            symbol: "test".into(),
            minter: minter,
            collection_info: sg721::CollectionInfo {
                creator: creator_addr,
                description: "todo!()".into(),
                image: "https://example.com/image.png".into(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            }
        }).unwrap(),
        funds: vec![],
        label: "collection".into(),
    });

    chain.orc.execute(
        PROXY_NAME,
        "proxy_collection_instantiate",
        &cw1::Cw1ExecuteMsg::Execute { 
            msgs: vec![init_msg] 
        },
        key,
        vec![],
    )
}



pub fn mint_token(
    chain: &mut Chain,
    collection: String,
    owner: String,
    key: &SigningKey,
) -> Result<ExecResponse, ProcessError> {

    let mint_msg = sg721_base::ExecuteMsg::Mint { 
        token_id: "1".into(), 
        owner, 
        token_uri: None, 
        extension: None
    };


    chain.orc.execute(
        PROXY_NAME,
        "proxy_token_mint",
        &cw1::Cw1ExecuteMsg::<Empty>::Execute { 
            msgs: vec![WasmMsg::Execute { 
                contract_addr: collection, 
                msg: to_binary(&mint_msg).unwrap(), 
                funds: vec![]
            }.into()] 
        },
        key,
        vec![],
    )
}



pub fn create_token_account(
    chain: &mut Chain,
    token_contract: String,
    token_id: String,
    pubkey: Binary,
    key: &SigningKey,
) -> Result<ExecResponse, ProcessError> {

    let chain_id = chain.cfg.orc_cfg.chain_cfg.chain_id.clone();

    let init_msg = cw83_tba_registry::msg::CreateInitMsg {
        pubkey,
        token_info: TokenInfo {
            collection: token_contract,
            id: token_id,
        },
    };

    let code_id = chain.orc.contract_map.code_id(ACOUNT_NAME)?;

    chain.orc.execute(
        REGISTRY_NAME, 
        "registry_create_account", 
        &cw83_tba_registry::msg::ExecuteMsg::CreateAccount(
            CreateAccountMsg {
                code_id,
                chain_id,
                msg: init_msg,
            }
        ), 
        key, 
        vec![]
    )
}


#[cw_serde]
pub struct FullSetupData {
    pub proxy: String,
    pub registry: String,
    pub collection: String,
    pub token_id: String,
    pub token_account: String,

    pub signer_mnemonic: String,
    pub public_key: Binary,

    pub user_address: String,
}


pub fn get_init_address(
    res: ChainTxResponse
) -> String {
    res
        .find_event_tags(
            "instantiate".to_string(), 
            "_contract_address".to_string()
        )[0].value.clone()
}


pub fn full_setup(
    chain: &mut Chain,
) -> Result<FullSetupData, ProcessError> {

    let _start_time = latest_block_time(chain).plus_seconds(60);


    let user: super::chain::SigningAccount = chain.cfg.users[0].clone();
    let user_address = user.account.address.clone();

    let reg_init = instantiate_registry(chain, user_address.clone(), &user.key).unwrap();
    
    let registry = get_init_address(reg_init.res);


    let proxy = instantiate_proxy(chain, user_address.clone(), &user.key).unwrap().address;
   
    let init_res = instantiate_collection(
        chain, 
        user.account.address.clone(), 
        proxy.clone().to_string(),
        &user.key
    ).unwrap();

    let collection  = get_init_address(init_res.res);

    
    let mint_res = mint_token(
        chain, 
        collection.clone(), 
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
        collection.clone(),
        token_id.clone(),
        pubkey.clone(),
        &user.key
    ).unwrap();


    let token_account = get_init_address(create_res.res);
    

    Ok(FullSetupData {
        proxy: proxy.to_string(),
        registry,
        collection,
        token_id,
        token_account,

        signer_mnemonic: user.account.mnemonic,
        public_key: pubkey,

        user_address
    })

}
 



pub fn wasm_query<S: Serialize>(
    chain: &mut Chain,
    address: &String,
    msg: &S
) -> Result<QueryResponse, CosmwasmError> {

    let res = tokio_block(async { 
        chain.orc.client.wasm_query(
            Address::from_str(&address)?,
            msg
        )
        .await }
    );

    res
}


pub fn query_token_owner(
    chain: &mut Chain,
    collection: String,
    token_id: String,
) -> Result<cw721::OwnerOfResponse, CosmwasmError> {

    let res = wasm_query(
        chain,
        &collection,
        &cw721::Cw721QueryMsg::OwnerOf {
            token_id, include_expired: None
        }
    ).unwrap();

    let owner_res : cw721::OwnerOfResponse = from_binary(
        &res.res.data.unwrap().into()
    ).unwrap();

    Ok(owner_res)
}
 


// gen_users will create `num_users` random SigningKeys
// and then transfer `init_balance` of funds to each of them.
pub fn gen_users(
    chain: &mut Chain,
    num_users: u32,
    init_balance: u128,
    denom: Option<&String>,
) -> Vec<SigningKey> {
    let prefix = &chain.cfg.orc_cfg.chain_cfg.prefix;
    let base_denom = &chain.cfg.orc_cfg.chain_cfg.denom;
    let from_user = &chain.cfg.users[1];

    let mut users = vec![];
    for n in 0..num_users {
        users.push(SigningKey::random_mnemonic(n.to_string()));
    }

    let mut reqs = vec![];
    for user in &users {
        let mut amounts = vec![OrcCoin {
            amount: init_balance,
            denom: base_denom.parse().unwrap(),
        }];
        // add extra denom if specified
        if let Some(denom) = denom {
            amounts.push(OrcCoin {
                amount: init_balance,
                denom: denom.parse().unwrap(),
            });
        }
        reqs.push(SendRequest {
            from: from_user.account.address.parse().unwrap(),
            to: user.to_addr(prefix).unwrap(),
            amounts,
        });
    }

    tokio_block(
        chain
            .orc
            .client
            .bank_send_batch(reqs, &from_user.key, &TxOptions::default()),
    )
    .unwrap();

    users
}

pub fn latest_block_time(chain: &Chain) -> Timestamp {
    let now = tokio_block(chain.orc.client.tendermint_query_latest_block())
        .unwrap()
        .block
        .header
        .unwrap()
        .time
        .unwrap();

    Timestamp::from_seconds(now.seconds.try_into().unwrap())
}



pub fn can_execute(
    chain: &mut Chain, 
    token_account: &String, 
    sender: String, 
    msg: CosmosMsg<Empty>
) -> CanExecuteResponse {
    let res = wasm_query(
        chain, 
        token_account, 
        &QueryMsg::CanExecute::<Empty> { 
            sender: sender, 
            msg: msg.into(), 
        }
    ).unwrap();
    
    from_binary(
        &res.res.data.unwrap().into()
    ).unwrap()
}