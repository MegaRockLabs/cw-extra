use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not Supported")]
    NotSupported {},

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}

impl From<OwnershipError> for ContractError {
    fn from(err: OwnershipError) -> Self {
        Self::Std(StdError::generic_err(err.to_string()))
    }
}

impl From<ParseReplyError> for ContractError {
    fn from(err: ParseReplyError) -> Self {
        Self::Std(StdError::GenericErr { msg: err.to_string() })
    }
}