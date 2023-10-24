use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, Addr, StdResult, WasmMsg, to_binary, Binary, Deps, QueryRequest, WasmQuery, from_binary, Empty};
use cw1::CanExecuteResponse;
use cw81::{ValidSignatureResponse, ValidSignaturesResponse};

use crate::{Cw82ExecuteMsg, Cw82QueryMsg};

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

    pub fn valid_signature(
        &self, 
        deps: Deps,
        data : Binary, 
        signature: Binary,
        payload: Option<Binary>
    ) -> StdResult<ValidSignatureResponse> {
        let wasm_query =  WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&Cw82QueryMsg::<Empty>::ValidSignature {
                data,
                signature,
                payload
            })?
        };
        let binary_res = deps.querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_binary(&binary_res)
    }


    pub fn valid_signatures(
        &self, 
        deps: Deps,
        data : Vec<Binary>, 
        signatures: Vec<Binary>,
        payload: Option<Binary>
    ) -> StdResult<ValidSignaturesResponse> {
        let wasm_query =  WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&Cw82QueryMsg::<Empty>::ValidSignatures {
                data,
                signatures,
                payload
            })?
        };
        let binary_res = deps.querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_binary(&binary_res)
    }


    pub fn can_execute(
        &self, 
        deps: Deps,
        sender : String, 
        msg: impl Into<CosmosMsg>,
    ) -> StdResult<CanExecuteResponse> {
        let wasm_query =  WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&Cw82QueryMsg::<Empty>::CanExecute {
                sender,
                msg: msg.into()
            })?
        };
        let binary_res = deps.querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_binary(&binary_res)
    }
}