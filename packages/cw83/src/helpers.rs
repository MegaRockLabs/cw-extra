use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, Addr, StdResult, WasmMsg, to_binary};


#[cw_serde]
pub struct Cw83RegistryContract(pub Addr);

impl Cw83RegistryContract {
    
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

}