use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Empty, Addr};
use cw83::{registy_execute, registy_query, 
    CreateAccountMsg as CreateAccountMsgBase,
    AccountQuery as AccountQueryBase,
    AccountInfoResponse as AccountInfoResponseBase,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Option<Vec<Addr>>,
    pub allowed_ids: Vec<u64>,
}

#[cw_serde]
pub struct TokenInfo {
    pub collection: String,
    pub id: String,
}

#[cw_serde]
pub struct CreateInitMsg {
    pub token_info: TokenInfo,
    pub pubkey: Binary,
}

#[cw_serde]
pub struct CollectionsResponse {
    pub collections: Vec<String>,

}

#[cw_serde]
pub struct Account {
    pub collection: String,
    pub id: String,
    pub address: String,
}

#[cw_serde]
pub struct CollectionAccount {
    pub id: String,
    pub address: String,
}


#[cw_serde]
pub struct AccountsResponse {
    pub total: u32,
    pub accounts: Vec<Account>
}

#[cw_serde]
pub struct CollectionAccountsResponse {
    pub total: u32,
    pub accounts: Vec<CollectionAccount>
}

pub type AccountQuery = AccountQueryBase<TokenInfo>;
pub type AccountInfoResponse = AccountInfoResponseBase<Empty>;
pub type CreateAccountMsg = CreateAccountMsgBase<CreateInitMsg>;


#[registy_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(AccountsResponse)]
    Accounts {
        skip: Option<u32>,
        limit: Option<u32>,
    },

    #[returns(CollectionAccountsResponse)]
    CollectionAccounts {
        collection: String,
        skip: Option<u32>,
        limit: Option<u32>,
    },

    #[returns(CollectionsResponse)]
    Collections {
        skip: Option<u32>,
        limit: Option<u32>
    },
}

#[cw_serde]
pub struct MigrateMsg {}


#[registy_execute]
#[cw_serde]
pub enum ExecuteMsg {

    UpdateAllowedIds {
        allowed_ids: Vec<u64>
    },

    UpdateAccountOwnership {
        token_info: TokenInfo,
        new_pubkey: Binary,
        update_for: Option<Addr>,
    },

    ResetAccount(CreateAccountMsg),

    MigrateAccount {
        token_info: TokenInfo,
        new_code_id: u64,
        msg: Binary
    },

    FreezeAccount {
        token_info: TokenInfo
    },

    UnfreezeAccount {
        token_info: TokenInfo
    },

}