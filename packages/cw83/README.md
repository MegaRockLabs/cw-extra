# CW83: Smart Account Registry

An interface for CosmWasm based smart contracts defining interaction with smart account registries used for creating smart contract based account defined in [CW82](/packages/cw82)


## Queries


All CW83-compliant registries must define a message with the following variants in for their query endpoint

```rust
enum QueryMsg {
    ...

   #[returns(AccountResponse)]
   AccountInfo( ** Your Custom Query Type ** ) // default is Option<Binary> / Binary
    ...
}
```
With `multi` feature flag enabled the `AccountInfo` there is an additional variant for searching multiple accounts at once

```rust
enum QueryMsg {

    ...

    #[returns(AccountResponse)]
    Accounts {
        query           :     ** Your Custom Query Type **
        start_after     :     Option<String>,
        skip            :     Option<u32>,
        limit           :     Option<u32>,
    }

    ...
}
```


The minimal information to send back as a response is the address of the snart acciybt if it exists. On top of that users are free to extend the response with any additional information they want to expose.  The response is defined as follows:
```rust
struct AccountResponse<T = Option<Empty>> {
    pub address : String,
    pub info    : T
}
```

### Macro

The crate of `cw83` exposes a macro attribute `registry_query` that injects the required variants with high degree of customization.  It takes optional positional arguments that modify ether the inner query type or the response type

#### Default Usage
Without additional feature flags the macro takes up to two optional positional arguments
```rust
#[registry_query(YourQueryType, YourResponseType)]
enum QueryMsg {}

//would correspond to the following code
enum QueryMsg {
    #[returns(AccountResponse<YourResponseType>)]
    AccountInfo(YourQueryType)
}
```
By default QueryType is set to Option<Binary> and ResponseType is set to Option<Empty>. 

#### With `multi` feature 

In this case we can use up to four positional arguments
```rust
#[registry_query(Query, RespInfo, MultiQuery, MultiResInfo)]
enum QueryMsg {}

//would correspond to the following code
enum QueryMsg {
    
    #[returns(AccountResponse<RespInfo>)]
    AccountInfo(Query),

    #[returns(AccountResponse<MultiResInfo>)]
    Accounts {
        query           :     MultiQuery,
        start_after     :     Option<String>,
        skip            :     Option<u32>,
        limit           :     Option<u32>,
    }
}
```

If not specified the `MultiResInfo` used the same type as `RespInfo`. 
`MultiQuery` uses `Option`:al form of `Query` type in case if it isn't an `Option` itself. In case of the latter it's assigned to the same type without nesting 




## Messages

The only required message variant for the execute endpoint is `CreateAccount``:

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
    pub account_data: T
}
```
allowing contracts to define payload needed for validation in the registry and also for generating an instantiation message for smart account contracts


### Macro

The crate of `cw83` exposes a macro attribute `registry_execute` that injects the required variant into your execute message. 

The macros takes one optional positional argument that allows to customise the type of the `msg` field in the `CreateAccountMsg` struct. By default it is set to `Binary` but can be changed to any other type. 

```rust
#[registry_execute(TokenAccount)]
enum ExecuteMsg {}

//would correspond to the following code
enum ExecuteMsg {
    ...
    CreateAccount(CreateAccountMsg<TokenAccount>)
    ...
}
```


It is also possible to use the minimal messages that have the variants of the staandard directly.  The easieast option is importing `Cw83QueryMsg` or `Cw83ExecuteMsg` from the crate



## Examples
Example contracts can be found in this repository and are prefixed with `cw83-`  

| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw83-tba-registry`](https://github.com/MegaRockLabs/cw-tba/tree/main/contracts/cw83-tba-registry)               | A Registry of token bound accounts                           |

