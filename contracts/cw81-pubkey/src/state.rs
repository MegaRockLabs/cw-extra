use cosmwasm_std::Binary;
use cw_storage_plus::Item;


pub static PUBKEY : Item<Binary> = Item::new("s");