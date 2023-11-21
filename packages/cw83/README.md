# CW83: Smart Account Registry

An interface for CosmWasm based smart contracts defining interaction with smart account registries used for creating smart contract based account defined in [CW82](/packages/cw82)


## Queries


All CW83-compliant registries must define a message with the following variants in for their query endpoint

```rust
enum QueryMsg {
    ...

   #[returns(AccountInfoResponse)]
   AccountInfo(AccountQuery)
    ...
}
```

Where AccountQuery is defined in the following manner

```rust
struct AccountQuery<T = Empty> {
    pub query: T
}
```
Different implementations are free to customise the query in any desirable manner to link an account to a username, a non-fungible token, credential info and so on.


The response type enforces the contracts implementing the standard to return a smart account **address** corresponsing to the query and leave a room to customize for returning addition info related to the account

```rust
struct AccountInfoResponse<T = Empty> {
    pub address: String,
    pub info: Option<T>
}
```


## Messages

The only requred message variant for the execute endpoint is the following:

```rust
enum ExecuteMsg {
    ...
    CreateAccount(CreateAccountMsg)
    ...
} 
```

where CreateAccountMsg is defined in the following manner:

```rust
struct CreateAccountMsg<T = Binary> {
    pub code_id: u64,
    pub chain_id: String,
    pub msg: T
}
```
allowing contracts to define payload needed for validation in the registry and also for generating an instantiation message for smart account contracts


## Usage

A contract that wishes to follow the standard must add the variants described above to their query and execute messages. This package exposes a helper macro attribute `registy_query` that injects it automatically:

```Rust
#[registy_query] // <- Must be before #[cw_serde]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {}
```

The module where the message is defined must ether import `AccountQuery` from cw83 package or to define it manually. Here is an example of customising it from token bound account registry:

```Rust
use cw83::AccountQuery as AccountQueryBase;

#[cw_serde]
pub struct TokenInfo {
    pub collection: String,
    pub id: String,
}

pub type AccountQuery = AccountQueryBase<TokenInfo>;
```


Defining execute message can also happen through a helper

```Rust
#[registy_execute]
#[cw_serde]
pub enum Cw83ExecuteMsg {}
```

In similar manner `CreateAccountMsg` must also be imported. An example of customizing a message from the tba-registry:

```rust
use cw83::CreateAccountMsg as CreateAccountMsgBase

#[cw_serde]
pub struct CreateInitMsg {
    pub token_info: TokenInfo,
    pub pubkey: Binary,
}

pub type CreateAccountMsg = CreateAccountMsgBase<CreateInitMsg>;
```


If a contract doesn't define additional variants it can directly use  `Cw83QueryMsg` and `Cw83ExecuteMsg` from the package directly

## Examples
Example contracts can be found in this repository and are prefixed with `cw83-`  

| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw83-tba-registry`](contracts/cw83-tba-registry)               | A Registry of token bound accounts                           |

