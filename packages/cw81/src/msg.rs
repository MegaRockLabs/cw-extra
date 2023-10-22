use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;


#[cw_serde]
pub struct ValidSignature {
    pub data: Binary,
    pub signature: Binary,
    pub payload: Option<Binary>
}

#[cw_serde]
pub struct ValidSignatures {
    pub data: Vec<Binary>,
    pub signatures: Vec<Binary>,
    pub payload: Option<Binary>
}


#[cw_serde]
pub struct ValidSignatureResponse {
    pub is_valid: bool
}

#[cw_serde]
pub struct ValidSignaturesResponse {
    pub are_valid: Vec<bool>
}