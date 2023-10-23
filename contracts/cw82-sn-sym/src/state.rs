use cosmwasm_std::Binary;
use cosmwasm_storage::{singleton, singleton_read};


const SIGNING_KEY     : &[u8; 2]  = b"sk";
const DECRYPTING_KEY  : &[u8; 2]  = b"dk";


pub enum KeyType { Signing, Decrypting }


pub fn read_private(storage: &dyn cosmwasm_std::Storage, key_type: KeyType) -> Binary {

    let s = match key_type {
        KeyType::Signing => SIGNING_KEY,
        KeyType::Decrypting => DECRYPTING_KEY
    };

    singleton_read(storage, s).load().unwrap()
}

pub fn save_private(
    storage: &mut dyn cosmwasm_std::Storage, 
    key: &Binary,
    key_type: KeyType
) ->  Result<(), cosmwasm_std::StdError> {

    let s = match key_type {
        KeyType::Signing => SIGNING_KEY,
        KeyType::Decrypting => DECRYPTING_KEY
    };

    singleton(storage, s).save(key)
}
