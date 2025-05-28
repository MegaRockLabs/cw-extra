use cosmwasm_schema::{cw_serde, QueryResponses, serde::Serialize};
use types::wasm::{Empty, Binary};


/// Absolute minimum of a query that follows cw83 standard
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw83QueryMsg<InnerQuery = Option<Empty>> {
    
    /// Query information about a single account
    #[returns(AccountResponse)]
    AccountInfo(InnerQuery),

    /// Query information about multiple accounts at the same time
    #[cfg(feature = "multi")]
    #[returns(AccountsResponse)]
    Accounts {
        /// Filter parameters 
        query         :    InnerQuery,
        /// Number of accounts to return
        limit         :    Option<u32>,
        /// Address or any other id to start after. Mutually exclusive with `skip`
        start_after   :    Option<String>,
        /// Numeric argument for skipping accounts. Mutually exclusive with `start_after``
        skip          :    Option<u32>,
    }
}

/// Absolute minimum of a message that follows cw83 standard
#[cw_serde]
pub enum Cw83ExecuteMsg<M = Binary> {
    /// Create a new account
    CreateAccount(CreateAccountMsg<M>)
}



/// Inner type of ExecuteMsg::CreateAccount defined by cw83 for account registries
#[cw_serde]
pub struct CreateAccountMsg<T = Binary> {
    /// Code id of smart account to instantiate
    pub code_id: u64,
    /// Chain identifier of a network where the account will be created
    pub chain_id: String,
    /// Payload data for account configuration
    pub account_data: T
}



/// Response type for QueryMsg::AccountInfo defined by cw83 for account registries
#[cw_serde]
pub struct AccountResponse<T : Serialize = Option<Empty>> {
    /// smart contract address of the matched account
    pub address: String,
    /// additional information about the account
    pub info: T
}


/// Response type for QueryMsg::Accounts {
///    query         :    T,
///    start_after   :    Option<String>,
///    skip          :    Option<u32>,
///    limit         :    Option<u32>,
///} defined by cw83 for account registries
#[cfg(feature = "multi")]
#[cw_serde]
pub struct AccountsResponse<T : Serialize = Option<Empty>> {
    /// List of the accounts matching the query
    pub accounts: Vec<AccountResponse<T>>,
    /// Total number of accounts in the registry 
    pub total: u32,
}

