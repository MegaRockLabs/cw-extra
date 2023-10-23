use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, Addr, StdResult, WasmMsg, to_binary};

use crate::Cw82ExecuteMsg;

#[cw_serde]
pub struct Cw82Contract(pub Addr);

impl Cw82Contract {
    
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn execute<T: Into<Vec<CosmosMsg>>>(&self, msgs: T) -> StdResult<CosmosMsg> {
        let msg = Cw82ExecuteMsg::Execute { msgs: msgs.into() };
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
            funds: vec![],
        }
        .into())
    }
}