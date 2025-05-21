use cosmwasm_schema::cw_serde;
use types::wasm::{Item, Binary};
use cw_utils::Expiration;


#[cw_serde]
pub struct  SignatureState {
    pub signature: Binary,
    pub expiration: Expiration
}


pub static SIGNATURE_STATE : Item<SignatureState> = Item::new("s");