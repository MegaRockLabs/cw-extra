use cosmwasm_std::{Binary, StdError};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw23::{valid_signature_query, ValidSignatureResponse, ValidSignaturesResponse};
use thiserror::Error;


#[cw_serde]
pub struct InstantiateMsg {}


#[valid_signature_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    Signature { to_sign : Binary }
}


#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
