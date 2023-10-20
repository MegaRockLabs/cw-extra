use cosmwasm_std::Binary;
use cosmwasm_storage::{singleton, singleton_read};


const PRIV_KEY  : &[u8; 2]  = b"pr";


pub fn read_private(storage: &dyn cosmwasm_std::Storage) -> Binary {
    singleton_read(storage, PRIV_KEY).load().unwrap()
}

pub fn save_private(
    storage: &mut dyn cosmwasm_std::Storage, 
    key: &Binary
) ->  Result<(), cosmwasm_std::StdError> {
    singleton(storage, PRIV_KEY).save(key)
}
