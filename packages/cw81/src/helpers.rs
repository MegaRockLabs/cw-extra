use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, Binary, Deps, WasmQuery, to_binary, QueryRequest, from_binary};

use crate::{ValidSignatureResponse, Cw81QueryMsg, ValidSignaturesResponse};

#[cw_serde]
pub struct Cw81Contract(pub Addr);

impl Cw81Contract {
    
    pub fn addr(&self) -> Addr {
        self.0.clone()
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
            msg: to_binary(&Cw81QueryMsg::ValidSignature {
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
            msg: to_binary(&Cw81QueryMsg::ValidSignatures {
                data,
                signatures,
                payload
            })?
        };
        let binary_res = deps.querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_binary(&binary_res)
    }
}