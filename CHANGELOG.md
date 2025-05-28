# CHANGELOG

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
Project **TRIES** adhering to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html), however going through the active development stage and can't guarantee it (FOR NOW).

<!-- next-header -->

## [Unreleased] 

## Added
- support for both `cosmwasm 2.0` and `cosmwasm 1.x` at the same time
- added a local fork of [`cw22`](https://github.com/aura-nw/cw-plus/tree/main/packages/cw22) for convenience and dependency compatibility

- [`cw81`]: added `multi` feature tag to hide some of the variants and types

- [`cw82`]: a macro for messages `account_execute` that injects the required variant into your execute message
- [`cw82`]: additional types and variants for `multi` feature tag

- [`cw83`]: ability to customize the type of `account_data` in `CreateAccountMsg` by passing an argument to `registry_execute` macro 
- [`cw83`]: ablity to customise the type of `info` in `AccountResponse` by passing an argument to `registry_query` macro
- [`cw83`]: additional types and variants for `multi` feature tag


## Changed
- replaced 3 smaller proto crates with one global 

- [`cw81`]: the variant `ValidSignatures`  became optional for queries and is injected with `multi` feature tag only

- [`cw82`]: the variant `ValidSignatures`  became optional for queries and is injected with `multi` feature tag only

- [`cw82`]: renamed `smart_account_query` to `account_query` 

- [`cw83`]: the property `msg` in `CreateAccountMsg` was renamed to `account_data`


## Fixed
-- [`cw82`]: the query macro doesn't break if the message doesn't have a generic `T` 

