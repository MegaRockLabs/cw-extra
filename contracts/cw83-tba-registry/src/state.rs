use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};
use crate::msg::TokenInfo;


#[cw_serde]
pub struct AdminList {
    pub admins: Vec<Addr>,
}

impl AdminList {
    pub fn is_admin(&self, addr: &str) -> bool {
        self.admins.iter().any(|a| a.as_ref() == addr)
    }
}

pub static COL_TOKEN_COUNTS  : Map<&str, u32>               = Map::new("c");
pub static TOKEN_ADDRESSES   : Map<(&str, &str), String>    = Map::new("t");
pub static LAST_ATTEMPTING   : Item<TokenInfo>              = Item::new("l");
pub static ALLOWED_IDS       : Item<Vec<u64>>               = Item::new("i");
pub static ADMINS            : Item<AdminList>              = Item::new("a");