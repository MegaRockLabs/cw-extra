# CW82: Minimal Smart Account

The standard defining minimal interface for interacting with a smart contract based on CosmWasm. Contract implementing the standard might be both fully fledged accounts serving as a replacement for externally owned accounts (EOAs) or accounts of ephemeral nature and/or limited set of features meant to be used for the needs of an individual application. 

The standard doesn't introduce any new primitives and is achievable through a combination of multiple existing standards, mainly:


| Standard                                                               | Description                               |
| ---------------------------------------------------------------------- | ------------------------------------------|
| [`cw-1`](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw1)   | Proxy Contracts                           |
| [`cw-2`](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw2)   | Contract Info                             |
| [`cw-81`](/packages/cw81)                                              | Signature Verification                    |
| [`cw-22`](/packages/cw22)                                              | Supported Interface                       |                

## Queries

A final query message looks in the following manner. ValidSignatures (in plural) and requires `multi` feature flag to be enabled


```rust
pub enum QueryMsg<T = Empty> {
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
    
    // cw81 (optional) and requires `multi` feature flag
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

The package expose `#[account_query]` macro attribute that injects the variants into a your query message. There is also a default `Cw82QueryMsg` thay can be extended. The macro takes an optional argument to override the `Binary` type used in `payload` field. Check [`cw81`](/packages/cw81) for the examples.  The name `T` for the template parameter for your `QueryMsg` is special and will be used down to the CosmosMsg type. Using any other letter will not work and it will default to `Empty` type.

```rust

## Messages
The only message action the standard enforces is
```rust
enum ExecuteMsg<T = Empty> {
    ...

    Execute { msgs: Vec<CosmosMsg<T>> },
    
    ...
}
```
coming from `cw1`standard. Keep in mind that 
```rust
T: Clone + fmt::Debug + PartialEq + JsonSchema
```

The package provides a default `Cw82ExecuteMsg` alias and `#[account_execute]` macro attribute that injects the variant into your execute message.

Both macros anticipate a `QueryMsg` or `ExecuteMsg` to have a template parameter `T` that is used to customize the type of the inner cosmos messages. Keep in mind that they only recognize the letter T and will not work with other letters 


## Examples
Example contracts can be found in this repository and are prefixed with `cw81-`  

| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw82-key-account`](/contracts/cw82-key-account/)               | Signatures are verified against secp256k1 public key and all executable cosmos message must be signed by corresponding private key |
| [`cw82-token-account`](/contracts/cw82-token-account/)           | Only an NFT owner can execute some cosmos messages. Signature are checked against the stored public key through [direct sign](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-036-arbitrary-signature.md)    |
| [`cw82-sn-sym`](https://github.com/MegaRockLabs/cw-extra/tree/secret-network/contracts/cw82-sn-sym)      | Secret Network specifc contract that only allow cosmos messages that were encrypted by a secret key provided to the contract by instantiator. Signatures must be coming from a separate key generated inside the contract  |

