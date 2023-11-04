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
    pub contract: String,
    pub id: String,
}

#[cw_serde]
pub struct CreateInitMsg {
    pub token_info: TokenInfo,
    pub pubkey: Binary,
}


#[cw_serde]
pub struct CollectionAccount {
    pub id: String,
    pub address: String,

}

pub type CollectionAccountsResponse = Vec<CollectionAccount>;


pub type AccountQuery = AccountQueryBase<TokenInfo>;
pub type AccountInfoResponse = AccountInfoResponseBase<Empty>;
pub type CreateAccountMsg = CreateAccountMsgBase<CreateInitMsg>;


#[registy_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(CollectionAccountsResponse)]
    CollectionAccounts {
        collection: String,
        start_after: Option<u32>,
        limit: Option<u32>,
    },
}


#[registy_execute]
#[cw_serde]
pub enum ExecuteMsg {

    UpdateAllowedIds {
        allowed_ids: Vec<u64>
    },

    UpdateAccountOwnership {
        token_info: TokenInfo,
        new_pubkey: Binary 
    },

    FreezeAccount {
        token_info: TokenInfo
    },

    UnfreezeAccount {
        token_info: TokenInfo
    },

}