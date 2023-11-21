# CW82: Smart Contract Account

The standard defining minimal interface for interacting with a smart contract based on CosmWasm. Contract implementing the standard might be both fully fledged accounts serving as a replacement for externally owned accounts (EOAs) or accounts of ephemeral nature and/or limited set of features meant to be used for the needs of an individual application. 

The standard doesn't introduce any new primitives and achieved through a combination of multiple existing standards, mainly:


| Standard                                                               | Description                               |
| ---------------------------------------------------------------------- | ------------------------------------------|
| [`cw-1`](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw1)   | Proxy Contracts                           |
| [`cw-2`](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw2)   | Contract Info                             |
| [`cw-81`](/packages/cw81)                                              | Signature Verification                    |
| [`cw-22`](https://github.com/aura-nw/cw-plus/tree/main/packages/cw22)  | Supported Interface      (in-review)      |                

## Queries

A final query message looks in the following manner

```rust
pub enum QueryMsg {
    ...
    // cw1
    #[returns(CanExecuteResponse)]
    CanExecute { sender: String, msg: CosmosMsg<T> },
    
    // cw81
    #[returns(ValidSignatureResponse)]
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<Binary>
    },
    
    // cw81
    #[returns(ValidSignaturesResponse)]
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,
        payload: Option<Binary>
    }

    ...
}
```
`cw2`and `cw22`operate on storage level and use raw queries and do not enforce any variants

The package expose `#[smart_account_query]` macro attribute that injects the variants into a custom query message enum and also exposes a `Cw82QueryMsg<T = Empty>` that can be extended to to check an executional validity of any custom CosmosMsg

## Messages
The only execution message the standard enforces is
```rust
enum ExecuteMsg<T = Empty>
    ...
    Execute { msgs: Vec<CosmosMsg<T>> },
.    ...
}
```
coming from `cw1`standard. Keep in mind that 
```rust
T: Clone + fmt::Debug + PartialEq + JsonSchema
```

The package provides an `Cw82ExecuteMsg` alias and no macro attributes



## Examples
Example contracts can be found in this repository and are prefixed with `cw81-`  

| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw82-key-account`](/contracts/cw82-key-account/)               | Signatures are verified against secp256k1 public key and all executable cosmos message must be signed by corresponding private key |
| [`cw82-token-account`](/contracts/cw82-token-account/)           | Only an NFT owner can execute some cosmos messages. Signature are checked against the stored public key through [direct sign](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-036-arbitrary-signature.md)    |
| [`cw82-sn-sym`](https://github.com/MegaRockLabs/cw-extra/tree/secret-network/contracts/cw82-sn-sym)      | Secret Network specifc contract that only allow cosmos messages that were encrypted by a secret key provided to the contract by instantiator. Signatures must be coming from a separate key generated inside the contract  |

