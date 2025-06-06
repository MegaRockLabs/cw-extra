#[cfg(all(not(feature = "cosmwasm_v1"), not(feature = "cosmwasm"), not(feature = "secretwasm")))]
compile_error!("must enable one of the features: `cosmwasm_v1`, `cosmwasm`, `secretwasm`");
#[cfg(all(feature = "cosmwasm_v1", not(feature = "cosmwasm"), not(feature = "secretwasm")))]
pub use {cosmwasm_std_one as cosmwasm_std, cw_storage_plus_one as cw_storage_plus};
#[cfg(all(feature = "secretwasm", not(feature = "cosmwasm"), not(feature = "cosmwasm_v1")))]
pub use secretwasm_std as cosmwasm_std;
#[cfg(any(feature = "cosmwasm", feature = "cosmwasm_v1"))]
pub use {cw_storage_plus::{Map, Item}, cosmwasm_std::to_json_binary};
pub use cosmwasm_std::*;