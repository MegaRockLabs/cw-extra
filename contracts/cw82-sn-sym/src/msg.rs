use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Binary};


#[cw_serde]
pub struct  EncryptedMsg {
    pub encrypted_msg : Binary
}

impl From<EncryptedMsg> for CosmosMsg::<EncryptedMsg> {
    fn from(msg: EncryptedMsg) -> Self {
        CosmosMsg::<EncryptedMsg>::Custom(msg)
    }
}



#[cw_serde]
pub struct ValidSignatureResponse {
    pub is_valid: bool
}

#[cw_serde]
pub struct ValidSignaturesResponse {
    pub are_valid: Vec<bool>
}

#[cw_serde]
pub struct CanExecuteResponse {
    pub can_execute: bool,
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg <T = EncryptedMsg> {
    #[returns(Binary)]
    Signature { to_sign : Binary },

    #[returns(CanExecuteResponse)]
    CanExecute { sender: String, msg: CosmosMsg<T> },

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
    },


}

#[cw_serde]
pub struct InstantiateMsg {
    pub secret_key : Binary
}

#[cw_serde]
pub enum ExecuteMsg {
    Execute { msgs: Vec<CosmosMsg<EncryptedMsg>> },
}
