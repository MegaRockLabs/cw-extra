use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_json, to_json_binary, Addr, Binary, QuerierWrapper, QueryRequest, StdResult, WasmQuery};

use crate::{ValidSignatureResponse, Cw81QueryMsg, ValidSignaturesResponse};

pub const INTERFACE_NAME: &str = "crates:cw81";

#[cw_serde]
pub struct Cw81Contract(pub Addr);

impl Cw81Contract {
    
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn valid_signature(
        &self, 
        querier: &QuerierWrapper,
        data : Binary, 
        signature: Binary,
        payload: Option<Binary>
    ) -> StdResult<ValidSignatureResponse> {
        let wasm_query =  WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&Cw81QueryMsg::ValidSignature {
                data,
                signature,
                payload
            })?
        };
        let binary_res : Binary = querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_json(&binary_res)
    }


    pub fn valid_signatures(
        &self, 
        querier: &QuerierWrapper,
        data : Vec<Binary>, 
        signatures: Vec<Binary>,
        payload: Option<Binary>
    ) -> StdResult<ValidSignaturesResponse> {
        let wasm_query =  WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&Cw81QueryMsg::ValidSignatures {
                data,
                signatures,
                payload
            })?
        };
        let binary_res : Binary = querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_json(&binary_res)
    }

}