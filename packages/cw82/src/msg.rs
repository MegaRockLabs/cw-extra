use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Binary, Empty};
use cw1::{Cw1ExecuteMsg, CanExecuteResponse};

use schemars::JsonSchema;
use std::fmt;

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw82QueryMsg <T = Empty>
where T: Clone + fmt::Debug + PartialEq + JsonSchema {
    #[returns(CanExecuteResponse)]
    CanExecute { 
        sender: String, 
        msg: CosmosMsg<T> 
    },

    /// cw81
    #[returns(cw81::ValidSignatureResponse)]
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<Binary>
    },

    #[returns(::cw81::ValidSignaturesResponse)]
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,
        payload: Option<Binary>
    }
}



pub type Cw82ExecuteMsg<T> = Cw1ExecuteMsg<T>;