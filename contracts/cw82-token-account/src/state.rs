use cosmwasm_std::Binary;
use cw_storage_plus::Item;

use crate::msg::TokenInfo;

pub static REGISTRY_ADDRESS : Item<String>      = Item::new("r");
pub static TOKEN_INFO       : Item<TokenInfo>   = Item::new("t");
pub static PUBKEY           : Item<Binary>      = Item::new("p");
pub static OWNER            : Item<String>      = Item::new("o");

