use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Binary, Empty};

use cw1::{CanExecuteResponse, Cw1ExecuteMsg};
use cw2::ContractVersion;
use cw81::{ValidSignatureResponse, ValidSignaturesResponse};
use cw82_derive::{smart_account_query, extended_smart_account_query};

use schemars::JsonSchema;
use std::fmt;

#[smart_account_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw82QueryMsg <T = Empty>
where T: Clone + fmt::Debug + PartialEq + JsonSchema {}


#[extended_smart_account_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw82ExtendedQueryMsg <T = Empty>
where T: Clone + fmt::Debug + PartialEq + JsonSchema {}


pub type Cw82ExecuteMsg<T> = Cw1ExecuteMsg<T>;