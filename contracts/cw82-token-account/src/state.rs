use cosmwasm_std::Binary;
use cw_storage_plus::{Item, Map};

use crate::msg::{TokenInfo, Status};

pub static REGISTRY_ADDRESS : Item<String>      = Item::new("r");
pub static TOKEN_INFO       : Item<TokenInfo>   = Item::new("t");
pub static STATUS           : Item<Status>      = Item::new("s");
pub static PUBKEY           : Item<Binary>      = Item::new("p");
pub static MINT_CACHE       : Item<String>      = Item::new("m");

pub static KNOWN_TOKENS : Map<(&str, &str), bool>  = Map::new("k");

