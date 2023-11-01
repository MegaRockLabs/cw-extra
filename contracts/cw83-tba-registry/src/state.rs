use cosmwasm_schema::cw_serde;

use cw_storage_plus::{Map, Item};

#[cw_serde]
pub struct StoredAccount {
    pub owner: String,
    pub token_contract: String,
    pub token_id: String
} 


pub static STORED_ACCOUNTS : Map<&str, StoredAccount>   = Map::new("s");
pub static LAST_ATTEMPTING : Item<StoredAccount>        = Item::new("l");
pub static ALLOWED_IDS     : Item<Vec<u64>>             = Item::new("i");
pub static ADMIN           : Item<String>                 = Item::new("a");