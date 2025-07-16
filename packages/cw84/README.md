# CW84: Signed Actions [Draft]

The CW84 standard defines an interface for CosmWasm-based smart contracts that enables account abstraction through custom authentication and authorization flows using signed payloads. It extends the capabilities of smart accounts by introducing signed message execution and validation, building on existing standards to provide a flexible and secure framework for smart contract interactions.

This standard is particularly suited for smart accounts that require custom signature verification schemes, allowing for advanced use cases such as delegated execution, multi-signature workflows, and application-specific authentication logic. Contracts implementing CW84 are fully compatible with a suite of related standards, ensuring interoperability within the CosmWasm ecosystem.


### Related Standards

CW84-compliant contracts inherently support the following standards:


| Standard                                                               | Description                               |
| ---------------------------------------------------------------------- | ------------------------------------------|
| [`cw-1`](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw1)   | Proxy Contracts                           |
| [`cw-2`](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw2)   | Contract Info                             |
| [`cw-81`](/packages/cw81)                                              | Signature Verification                    |
| [`cw-82`](/packages/cw82)                                              | Minimal Smart Account                     |
| [`cw-22`](/packages/cw22)                                              | Supported Interface                       |                


### Interface

The CW84 standard defines a set of query and execute messages to support signed actions. The package provides procedural macros (#[signed_query] and #[signed_execute]) to inject the required variants into user-defined query and execute enums respectively. The package also expose minimal messages that implement the interface


## Execute Messages

The execute message that follows `cw84` must include the standard `Execute` variant imposed by both `cw82` and `cw81` and a new variant `ExecuteSigned` that is a unique to this standard. To optimise the usage of resources and make your life easier it it's advisable to define a separate message e.g. `ActionMsg` that contains all the variant that can be execute this way

```rust
pub enum ActionMsg {
    // Define your actions here
    Foo { },

    Bar { },

    Execute { msgs: Vec<CosmosMsg> },
}

pub enum ExecuteMsg {
    // cw82 | cw1
    Execute { 
        msgs: Vec<CosmosMsg>,
        signed: Option<Binary>,  // Optional signature data for enhanced security
    },

    // cw84
    ExecuteSigned {
        msg   : ActionMsg,
        signed  : Binary,
        nonce: Option<Uint64>,  // Optional nonce for replay protection
    },
}
```
If you don't want to have a separate message it's also possible to inlcude the `ExecuteMsg` itself but you might need to have it inside a box and possibly deal deal with nested structure later

```rust
pub enum ExecuteMsg {
    // ...

    ExecuteSigned {
        msg: Box<ExecuteMsg>,
        signed: Binary,
        nonce: Option<Uint64>,  // Optional nonce for replay protection
    }
}
``` 

### Multi

With `multi` feature enabled, the `msg` field becomes a plural `msgs` and the previos message type turnig into a collection (Vec)

```rust
pub enum ExecuteMsg {
    // ...

    ExecuteSigned {
        msgs: Vec<ActionMsg>, // or Vec<ExecuteMsg>, Box isn't necessary in this case
        signed: Binary,
        nonce: Option<Uint64>,  // Optional nonce for replay protection
    }
}
```

In case you went for separating signable actions into a separate message type like `ActionMsg` the standards suggests on adding an additional variant for doing multiple actions through the native flow
```rust
pub enum ExecuteMsg {
    // ...

    ExecuteNative {
        msgs: Vec<ActionMsg>,
    },
}
```
Without a separation or when including `Self` it defeats the purpose and as an optional variant it's for the better to omit it

### Signed Data

The `signed` field is a binary payload that can be used to pass additional information required for signature verification or other custom logic. However the contracts aren't required to use `cosmwasm_std::Binary` which is used by default and can replace it with any signed data object that fits their needs. Take a look at [SignedDataMsg](https://github.com/MegaRockLabs/smart-account-auth/blob/f2bb5a07a9683be144185f531d1227fc4c7ccb02/packages/common/src/types/msgs.rs#L20) from `smart-account-auth` if you look for potential options.


### Macros
**signed_execute**: Primary effect is injects both variants (`Execute` and `ExecuteSigned`) described above into your enum. Takes up to two positional arguments to customise the behaviour. The first argument is used to set the message type allowed to be signed like `ActionMsg` from above. The second argument is used to customise the type of `signed` to use something other than `cosmwasm_std::Binary`

With both `multi` feature tag active and first positional argument passed the macro also injects the plural variant `ExecuteNative` as described above.

The `Execute` variant now includes an optional `signed` field for enhanced security, and both `ExecuteSigned` variants include an optional `nonce` field for replay protection.

#### Default Usage

```rust
use cw84::signed_execute;
use cosmwasm_schema::cw_serde;

#[signed_execute]
#[cw_serde]
enum ExecuteMsg {}
```

which is equivalent to:

```rust
enum ExecuteMsg {

    Execute { 
        msgs: Vec<CosmosMsg>,
        signed: Option<Binary>,
    },

    ExecuteSigned {
        msg: ActionMsg,  
        signed: Binary,
        nonce: Option<Uint64>,
    },
}
```


#### Customised Usage (multi)


```rust
use cw84::signed_execute;
use cosmwasm_schema::cw_serde;

#[signed_execute(ActionMsg, SignedDataMsg)]
#[cw_serde]
enum ExecuteMsg {}
```

Which is equivalent to:

```rust
enum ExecuteMsg {

    Execute { 
        msgs: Vec<CosmosMsg>,
        signed: Option<SignedDataMsg>,
    },

    ExecuteSigned {
        msgs: Vec<ActionMsg>, 
        signed: SignedDataMsg,  // or Binary if you didn't pass the second argument
        nonce: Option<Uint64>,
    },

    // If `multi` feature is enabled and first argument is passed
    ExecuteNative {
        msgs: Vec<ActionMsg>
    },
}
```




## Query Messages

The query interface includes variants for checking execution permissions and validating signatures, with support for single and multi-signature verification via the multi feature. The Cw84QueryMsg enum defines the minimum required queries.

`cw84` requires all the compatible contracts have all the query variants imposed by `cw82`, namely:
- CanExecute: Verifies if a CosmosMsg can be executed by the specified sender, inherited from cw1 and cw81.
- CanExecuteNative: Verifies if a CosmosMsg can be executed natively (injected by macros when arguments are provided).
- ValidSignature: Validates a single signature against provided data and an optional payload, inherited from cw81.
- ValidSignatures: Validates multiple signatures against a list of data and an optional payload, available with the multi feature, also from cw81.

In addition to them all the compliant contracts must add the following new query variant:
```rust
enum QueryMsg {
    #[returns(::cw1::CanExecuteResponse)]
    CanExecuteSigned {
        msg:  ExecuteMsg, 
        signed: Binary,
        nonce: Option<Uint64>,  // Optional nonce for replay protection
    }
}
```
### Customisation
The feature `multi`, the type of `signed` field behaves identical to what was described in the `ExecuteMsg` section. The response object when `multi` is enabled is `CanExecuteSignedResponse` that contains vector of booleans indicating whether each message can be executed or not.

The `nonce` field provides replay protection and is included in both single and multi variants of `CanExecuteSigned`.

```rust
#[cw_serde]
struct CanExecuteSignedResponse {
    pub can_execute: Vec<bool>,
}
```

### Macros
**signed_query** macro injects all the necessary methods into your enum. It takes a mandatory positional argument that indicates an `ExecuteMsg` or even better `ActionMsg` defined by your contract. It also takes up to two additional type arguments. First one is used to set the type of `signed` field, which defaults to `cosmwasm_std::Binary`. The second one is customise `payload` for `cw81` queries (`ValidSignature`)


#### Default Usage

```rust
use cw84::signed_query;
use cosmwasm_schema::{cw_serde, QueryResponse};

#[signed_query(ActionMsg)]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {}
```

which is equivalent to:

```rust
enum QueryMsg {
     // cw84
    CanExecuteSigned {
        msg: ActionMsg, 
        signed: Binary,
        nonce: Option<Uint64>,
    },

    // cw82 | cw1
    CanExecute {
        sender: String,
        msg: CosmosMsg, 
    },

    // cw82 | cw1 (injected when arguments provided)
    CanExecuteNative {
        sender: String,
        msg: CosmosMsg,
    },

    // cw81
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<Binary>,
    },
}
```

#### Customised Usage (multi)

```rust
use cw84::signed_query;
use cosmwasm_schema::{cw_serde, QueryResponse};


#[signed_query(ActionMsg, SignedDataMsg, AuthPayload)]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {}
```

which is equivalent to:

```rust
enum QueryMsg {
    // cw84
    CanExecuteSigned {
        msg: ActionMsg, 
        signed: SignedDataMsg,
        nonce: Option<Uint64>,
    },

    // cw82 | cw1
    CanExecute {
        sender: String,
        msg: CosmosMsg, 
    },

    // cw82 | cw1 (injected when arguments provided)
    CanExecuteNative {
        sender: String,
        msg: CosmosMsg,
    },

    // cw81
    ValidSignature {
        data: Binary,
        signature: Binary, 
        payload: Option<AuthPayload>, 
    },
    // cw81-multi
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,  
        payload: Option<AuthPayload>, 
    },
}
```

## References 

Examples to be used as a reference or used directly to avoid reinventing the wheel and give the best compatibility with existing ecosystem

### Messages

- [ExecuteMsg](http://github.com/AbstractSDK/abstract/blob/release-0.25/framework/packages/abstract-std/src/account.rs#L133) from the Account contract by **AbstrackSdk** 
- [AuthenticatorSudoMsg](https://github.com/osmosis-labs/cw-authenticator/blob/88478228a937a5344d9ae661e273aceed391311c/src/types.rs#L91) by **Osmosis** for account contracts using x/smart-account module


### Signed Data

There are multiple possible options that are all defined in `smart-account-auth` authentication library:
- [Credential](https://github.com/MegaRockLabs/smart-account-auth/blob/f2bb5a07a9683be144185f531d1227fc4c7ccb02/packages/bundle/src/credential.rs#L21) a wrapper over all supported credential types that can be conditionally compiled to only include the desired ones
- [CredentialData](https://github.com/MegaRockLabs/smart-account-auth/blob/f2bb5a07a9683be144185f531d1227fc4c7ccb02/packages/bundle/src/data.rs) that wraps over batch of credentials at the same time with utility methods to work with them
- [Box\<dyn Verifiable\>](https://github.com/MegaRockLabs/smart-account-auth/blob/f2bb5a07a9683be144185f531d1227fc4c7ccb02/packages/common/src/traits.rs#L5) or optionally boxed `CredentialsWrapper` both of which represent more general version of the first two options respectively.
- [SignedDataMsg](https://github.com/MegaRockLabs/smart-account-auth/blob/f2bb5a07a9683be144185f531d1227fc4c7ccb02/packages/common/src/types/msgs.rs#L20) to build a credential from stored info and new payload 
- [AuthPayload](https://github.com/MegaRockLabs/smart-account-auth/blob/f2bb5a07a9683be144185f531d1227fc4c7ccb02/packages/common/src/types/msgs.rs#L5) for default behaviour dicated by contract with minor tweaking or completely custom behaviour dictated by binary `extension` field

There are also the following options from different sources:
- [SignatureData](https://github.com/osmosis-labs/cw-authenticator/blob/main/src/types.rs#L75) and other types from `cw-authenticator`of **Osmosis**
- [Binary](https://github.com/CosmWasm/cosmwasm/blob/main/packages/std/src/binary.rs#L18) from `cosmwasm_std` 

Additionaly we can define a new type combine any of the abobe
- as an enum of a tuple type to pick one of the options
- as a struct with optional fields to ether pick one or to somehow combine
