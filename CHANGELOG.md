# CHANGELOG

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
Project **TRIES** adhering to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html), however going through the active development stage and can't guarantee it (FOR NOW).

<!-- next-header -->

## [Unreleased] Rust

## Added
- brought back testing folder that depends on `cw-auths` crate

## Removed
- `injective` feature tag until implemented

## Fixed
-- `to_json_string` imports and definitions


## [0.25.0] Rust - 2024-12-18 

## Added

- new types, trairs and other primitives for session keys and derivable actions. Available under new `session` feature tag
- a separate testing package for lib tests
- new feature tags that allow to optionally include additional inner `types`, `traits`, and `utils`
- minimalastic and vm agnostic versions of `Timestamp`, `Uint64` and `Uint128` from `cosmwasm_std`
- minimalastic and vm agnostic versions of `Expiration` and `Duration` from `cw-utils`
- added macros `saa-error` and `saa_derivable` for internal use
- this document

## Changed
- type of CredentialId has changed from `Vec<u8>` to `String`
- response types that had ids were also changed from  `Binary` to `String` 
- `Caller` credential is now an enum struct (after being a regular struct)
- Minor changes to `ClientData` of `PasskeyCredential` related to optional key fields
- renamed module with static storage variables from `storage` to `stores` 
- renamed `saa_type` to `saa_type` and added ability to omit exlusion of unknown properties by passing `no_deny`
- split type macros into separate files for each type
- `CredentialData`'s method was renamed to `with_native` and now inject the new caller from attached info only if the flag is set to `Some(true)` and return a miiror copy otherwise
- renamed `construct_credential` to `build_credential` and expose by default
- updated readme with features and focus-areas

## Removed
- All storage related types and primitives and logic have been removed to be moved to a separate package for each VM  
- Deleted  `storage` and `iterator` feature tags

## Fixed
- validation for max and min number of credentials in `CredentialData`
- fixed situatino with redundant (re-)validations 
- removed clutter from complex derrive clauses
- macros 


## [Unreleased] Typescript


## Changed
- stopped converting `PasskeyCredential::credential_data.challenge` from `base64` to `base64` 