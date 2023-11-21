use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ProxyInstantiateMsg {
    pub admins: Vec<String>,
    pub mutable: bool,
}
