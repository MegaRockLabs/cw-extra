mod msg;
mod interface;

pub use cw1::CanExecuteResponse;
pub use cw2::ContractVersion;
pub use cw81::{ValidSignatureResponse, ValidSignaturesResponse};
pub use cw82_derive::smart_account_query;

pub use msg::{Cw82QueryMsg, Cw82ExecuteMsg};
pub use interface::{Cw82Contract, INTERFACE_NAME};