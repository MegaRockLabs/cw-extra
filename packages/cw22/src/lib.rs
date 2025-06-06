//! CW22 defines a way for a contract to declare which interfaces do the contract implement
//! This standard is inspired by the EIP-165 from Ethereum. Originally it was proposed to
//! be merged into CW2: Contract Info, then it is splitted to a separated cargo to keep CW2
//! being backward compatible.

//! Each supported interface contains a string value pointing to the corresponding cargo package
//! and a specific release of the package. There is also a function to check whether the contract
//! support a specific version of an interface or not.

//! The version string for each interface follows Semantic Versioning standard. More info is in:
//! https://docs.rs/semver/latest/semver/
use cosmwasm_schema::cw_serde;
use types::wasm::{QuerierWrapper, QueryRequest, StdError, StdResult, Storage, WasmQuery, Map};
use semver::{Version, VersionReq};
use std::borrow::Cow;

pub const INTERFACE_NAMESPACE: &str = "supported_interfaces";
pub const SUPPORTED_INTERFACES: Map<&str, String> = Map::new(INTERFACE_NAMESPACE);


#[cw_serde]
pub struct ContractSupportedInterface<'a> {
    /// supported_interface is the name of an interface that the contract support.
    /// This is inspired by the EIP-165 from Ethereum.
    /// Interface names should follow a common standard such as <Registry Domain>:<Crate Name> in Rust crate registry.
    /// e.g. "crates.io:cw2"
    /// NOTE: this is just a hint for the caller to adapt on how to interact with this contract.
    /// There is no guarantee that the contract actually implement these interfaces.
    pub supported_interface: Cow<'a, str>,
    /// semantic version on release tags of the interface package following SemVer guideline.
    /// e.g.  "0.16.0"
    pub version: Cow<'a, str>,
}

/// set_contract_supported_interface should be used in instantiate to store the original version
/// of supported interfaces. It should also be used after every migration.
pub fn set_contract_supported_interface(
    store: &mut dyn Storage,
    supported_interfaces: &[ContractSupportedInterface],
) -> StdResult<()> {
    for item in supported_interfaces {
        let ver = Version::parse(&item.version);
        match ver {
            Ok(_) => {
                SUPPORTED_INTERFACES.save(
                    store,
                    &item.supported_interface,
                    &item.version.to_string(),
                )?;
            }
            Err(_) => {
                return Err(StdError::generic_err("Version's format is invalid"));
            }
        }
    }
    Ok(())
}


pub fn query_supported_interface_version(
    querier: &QuerierWrapper,
    contract_addr: &str, 
    interface_name: &str,
) -> StdResult<Option<String>> {

    let key = types::wasm::storage_keys::namespace_with_key(
        &[INTERFACE_NAMESPACE.as_bytes()], 
        interface_name.as_bytes()
    );

    let raw_query = WasmQuery::Raw { 
        contract_addr: contract_addr.into(),
        key: key.into()
    };

    querier.query(&QueryRequest::Wasm(raw_query))
}



pub fn minimum_version(version: &str, required: &str) -> bool {
    if let Ok(ver) = Version::parse(version) {
        if let Ok(req) = VersionReq::parse(format!(">={}", required).as_str()) {
            return req.matches(&ver);
        }
    }
    false
}

/// query_supported_interface show if contract supports an interface with version following SemVer query
/// query example">=1.2.3, <1.8.0"
pub fn require_version(version: &str, request: &str) -> bool {
    if let Ok(ver) = Version::parse(version) {
        if let Ok(req) = VersionReq::parse(request) {
            return req.matches(&ver);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::wasm::testing::MockStorage;

    #[test]
    fn get_and_set_work() {
        let mut store = MockStorage::new();

        let interface2 = "crates.io:cw2";
        let interface22 = "crates.io:cw22";
        let _interface721 = "crates.io:cw721";
        let contract_interface2 = ContractSupportedInterface {
            supported_interface: Cow::Borrowed(interface2),
            version: Cow::from("0.16.0"),
        };
        let contract_interface22 = ContractSupportedInterface {
            supported_interface: Cow::Borrowed(interface22),
            version: "0.1.0".into(),
        };
        let contract_interface721 = ContractSupportedInterface {
            supported_interface: Cow::Borrowed(interface22),
            version: Cow::from("v0.1.0"),
        };

        // set supported_interface error
        let supported_interface = &[contract_interface721];

        let rs_error =
            set_contract_supported_interface(&mut store, supported_interface).unwrap_err();
        let expected = StdError::generic_err("Version's format is invalid");
        assert_eq!(expected, rs_error);

        // set supported_interface
        let supported_interface = &[contract_interface2, contract_interface22];

        set_contract_supported_interface(&mut store, supported_interface).unwrap();
/*         // get version of not supported interface
        let loaded = query_supported_interface_version(&store, interface721).unwrap();
        assert_eq!(None, loaded);

        // get version of supported interface
        let loaded = query_supported_interface_version(&store, interface2).unwrap();
        let expected = String::from("0.16.0"); 
        assert_eq!(Some(expected), loaded);
        */
    }

    #[test]
    fn test_require_version() {
        let version_req = ">=0.1.0";
        let result = require_version("0.16.0", version_req);
        assert!(result);

        let version_req = ">=0.16.0";
        let result = require_version("0.1.0", version_req);
        assert!(!result);

        let version_req = ">=1.2.3, <1.8.0";
        let result = require_version("0.16.0", version_req);
        assert!(!result);

        let version_req = ">=0.2.3";
        let result = require_version("v0.16.0", version_req);
        assert!(!result);

        let version_req = "!=0.2.3";
        let result = require_version("0.16.0", version_req);
        assert!(!result);
    }

    #[test]
    fn test_minimum_version() {
        let result = minimum_version("0.16.0", "0.2.3");
        assert!(result);

        let result = minimum_version("0.2.0", "0.2.3");
        assert!(!result);

        let result = minimum_version("v0.16.0", "0.2.3");
        assert!(!result);

        let result = minimum_version("0.16.0", "v0.2.3");
        assert!(!result);
    }
}
