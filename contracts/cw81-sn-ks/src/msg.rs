use cosmwasm_std::{Binary, StdError};
use cosmwasm_schema::{cw_serde, QueryResponses};
use thiserror::Error;


#[cw_serde]
pub struct InstantiateMsg {}


#[cw_serde]
pub struct ValidSignatureResponse {
    pub is_valid: bool
}

#[cw_serde]
pub struct ValidSignaturesResponse {
    pub are_valid: Vec<bool>
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Binary)]
    Signature { to_sign : Binary },

    #[returns(ValidSignatureResponse)]
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<Binary>
    },

    #[returns(ValidSignaturesResponse)]
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,
        payload: Option<Binary>
    }
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
