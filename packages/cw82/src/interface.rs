use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, Addr, StdResult, WasmMsg, to_binary, Binary, QueryRequest, WasmQuery, from_binary, Empty, QuerierWrapper};
use cw1::CanExecuteResponse;
use cw81::{ValidSignatureResponse, ValidSignaturesResponse};

use crate::{Cw82ExecuteMsg, Cw82QueryMsg};

pub const INTERFACE_NAME : &str = "crates.io:cw82";

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
        querier: &QuerierWrapper,
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
            msg: to_binary(&Cw82QueryMsg::<Empty>::ValidSignatures {
                data,
                signatures,
                payload
            })?
        };
        let binary_res = querier.query(&QueryRequest::Wasm(wasm_query))?;
        from_binary(&binary_res)
    }


    pub fn can_execute(
        &self, 
        querier: &QuerierWrapper,
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