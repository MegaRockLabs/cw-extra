use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, Binary, WasmQuery, to_binary, QueryRequest, from_binary, QuerierWrapper};

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
            msg: to_binary(&Cw81QueryMsg::ValidSignature {
                data,
                signature,
                payload
            })?
        };
        let binary_res = querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_binary(&binary_res)
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
            msg: to_binary(&Cw81QueryMsg::ValidSignatures {
                data,
                signatures,
                payload
            })?
        };
        let binary_res = querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_binary(&binary_res)
    }

    pub fn supports_interface(
        &self,
        querier: &QuerierWrapper,
    ) -> StdResult<bool> {

        let key = cosmwasm_std::storage_keys::namespace_with_key(
            &[cw22::SUPPORTED_INTERFACES.namespace()], 
            INTERFACE_NAME.as_bytes()
        );

        let raw_query = WasmQuery::Raw { 
            contract_addr: self.addr().into(),
            key: key.into()
        };

        let version : Option<String> = querier.query(&QueryRequest::Wasm(raw_query))?;
        Ok(version.is_some())
    }
}