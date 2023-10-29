use super::chain::Chain;
use super::msg::ProxyInstantiateMsg;
use cosm_orc::orchestrator::cosm_orc::tokio_block;
use cosm_orc::orchestrator::deploy::DeployInfo;
use cosm_orc::orchestrator::error::ProcessError;
use cosm_orc::orchestrator::{Coin as OrcCoin, ExecResponse, Address};
use cosm_orc::orchestrator::{InstantiateResponse, SigningKey};
use cosm_tome::chain::request::TxOptions;
use cosm_tome::modules::bank::model::SendRequest;
use cosmwasm_std::{Timestamp, Addr, Empty, CosmosMsg, WasmMsg, to_binary};

use cw83_tba_registry::msg::InstantiateMsg;
use test_context::futures::TryFutureExt;

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

pub async fn instantiate_collection(
    chain: &mut Chain,
    creator_addr: String,
    key: &SigningKey,
) -> Result<ExecResponse, ProcessError> {
    // let infos: Vec<(String, DeployInfo)> =  chain.cfg.orc_cfg.contract_deploy_info.clone().into_iter().collect();
    
    let address : Address = key.to_addr(&chain.cfg.orc_cfg.chain_cfg.prefix).await.unwrap();
    let code_id = chain.orc.contract_map.code_id(COLLECTION_NAME)?;

    let init_msg : CosmosMsg::<Empty> = CosmosMsg::Wasm(WasmMsg::Instantiate { 
        admin: Some(address.to_string()), 
        code_id, 
        msg: to_binary(&sg721::InstantiateMsg {
            name: "test".into(),
            symbol: "test".into(),
            minter: address.to_string(),
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
        "collection_instantiate",
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
    chain.orc.execute(
        COLLECTION_NAME,
        "token_mint",
        &sg721::ExecuteMsg::<Empty, Empty>::Mint { 
            token_id: "1".into(), 
            owner, 
            token_uri: None, 
            extension: Empty {}
        },
        key,
        vec![],
    )
}




/* pub fn create_account(
    chain: &mut Chain,
    registry_addr: String,
    key: &SigningKey,
) -> Result<InstantiateResponse, ProcessError> {

    cw83::Cw83ExecuteMsg::CreateAccount { 
        code_id: (), 
        init_msg: (), 
        token_contract: (), 
        token_id: () 
    }

    chain.orc.execute(
        REGISTRY_NAME, 
        "registry_create_account", 
        msg, 
        key, 
        vec![]
    )

    cw83::Cw83RegistryContract(Addr::unchecked(registry_addr)).create_account_submsg(code_id, init_msg, token_contract, token_id, funds);

    chain.orc.instantiate(
        REGISTRY_NAME,
        "registry_inst",
        &InstantiateMsg {
            allowed_ids: vec![account_id],
        },
        key,
        Some(creator_addr.parse().unwrap()),
        vec![],
    )
}
 */
// gen_users will create `num_users` random SigningKeys
// and then transfer `init_balance` of funds to each of them.
pub async fn gen_users(
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
        users.push(SigningKey::random_mnemonic(n.to_string(), String::default()));
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
            to: user.to_addr(prefix).await.unwrap(),
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
