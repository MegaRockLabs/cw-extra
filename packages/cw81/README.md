# CW81: Signature Verification

With the adoption of smart contract based accounts comes the need to reliably communicate with them. That requires establishing ways to reliably retrieve certain pieces of information that was trivial to get from users using normal cryptographic pair based accounts 

This standard touches on the problem and proposes a standard way to verify that a signature belongs to a contract. Normally users would sign a message with their private keys, but since they are to be secret they can't be securely stored inside storage of a contract on most chains and therefore used for signing.

For that reason, the logic for saying that the signature is valid depends on the internal implementation of a contract. It can check validity based on input from an owner, use any key curve and signature schema, rely on oauth token verification and so on. 

Whatever the logic is we need primitives to verifyably query the results and that's the only thing that the standard is covering.


## Queries

All CW81-compliant contracts must add the following query variants to their QueryMsg':'s and return the corresponding responses (in binary format).
ValidSignatures (in plural) is an optinal variant to add and can be turn on by using the `multi` feature flag. 

The contracts are free to ovveride the type inside the option for `payload` field to fit their needs the best. The default is `Option<Binary>` which is a good fit for most cases.

```rust
pub enum QueryMsg {
    ...

    #[returns(cw81::ValidSignatureResponse)]
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<Binary> 
    },

    // Only if `multi` feature is enabled
    #[returns(cw81::ValidSignaturesResponse)]
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,
        payload: Option<Binary>
    }

    ...
}
```

### ValidSignature

Used to verify one message and its corresponding signature. Useful in atomic scenarios where a batch of data/transactions is combined together and treated as the same unit with one signature.

#### Fields

`data` that can include any transactional data, cosmos messages, arbitrary text, cryptographic digest or anything else. Unlike ERC-1271 the standard doesn't enforce the input to be pre-hashed but allows it. CosmWasm environment is more optimized for binary inputs and doesn't have as significant performance bottlenecks.

`signature` stands for signed bytes of the data field and doesn't enforce any conditions

`payload` is an optional payload used for passing additional info to the contract. It can be used to pass necessary data like a list of public keys for a multisig contract or for example information desctibing which credential to use in complex contracts. Things you might want to customise are e.g. signature schema to use, hashing algorithm to select or whether data had been serialized with a wrapper already.

#### Returns
```Rust 
struct ValidSignatureResponse {
  pub is_valid: bool
}
```
ERC-1271 introduces a `MAGICVALUE` as a return type instead of a boolean in order to have stricter and simpler verification of a signature. We can follow the same rationale, but instead of arbitrary bytes we can follow the existing conventions and return a struct


### ValidSignatures

In case of the need to verify multiple signatures at the same time, we can reduce the number of RPC requests by allowing them to be batched inside one QueryMsg. Some APIs already have methods for batch verification such as `deps.api.ed25519_batch_verify`in std/crypto library

A great example use-case would be is situation where a querier is satisfied with one of the messages being invalid and can proceed with the rest


#### Fields

`data` has identical meaning behind it except for including a list (vector) of messages, the signature of which we are checking

`signatures` contains a list of signatures for each message in the data list. Must have the same length.

`payload` field is identical to `ValidSignature` and can be used to pass additional information to the contract.

***
Technically doesn't need a different schema, but semantically works better this way. Since we use Binary (bytes) we even can use one field and serialize it to anything we need, but that doesn't provide additional intuitional utilities

#### Returns
```Rust 
struct ValidSignaturesResponse {
  pub are_valid: Vec<bool>
}
```

`are_valid` is a list of booleans that must be of the same length with `data` and `signatures` lists. It tells whether an individual signature for a corresponsing message was valid.


## Usage
A contract that wishes to follow the standard must add the variants described above to their query messages. This package exposes a helper macro attribute `valid_signature_query` that injects it automatically:

```Rust
#[valid_signature_query] // <- Must be before #[cw_serde]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {}
```

This will make it equivavlent to
```Rust
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {
    #[returns(ValidSignatureResponse)]
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<Binary>
    },

    #[returns(ValidSignaturesResponse)]
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,
        payload: Option<Binary>
    }
}
```

You can also pass an optional argument to the `valid_signature_query` macro to customize the optional type in the `payload` field:  
```Rust
# taken from `smart-account-auth`
struct AuthPayload {
    /// Which credential to use if multiple are available
    pub credential_id   :   Option<String>,
    /// Human readable prefix to use to derive an address
    pub hrp             :   Option<String>,
    /// Additional arguments to pass depending on a credential in question
    pub extension       :   Option<Binary>,
}

#[valid_signature_query(AuthPayload)] 
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {}
```

This is equivalent to the following:
```Rust
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {
    #[returns(ValidSignatureResponse)]
    ValidSignature {
        data: Binary,
        signature: Binary,
        payload: Option<AuthPayload>
    },

    #[returns(ValidSignaturesResponse)]
    ValidSignatures {
        data: Vec<Binary>,
        signatures: Vec<Binary>,
        payload: Option<AuthPayload>
    }
}
```



## Examples
Example contracts can be found in this repository and are prefixed with `cw81-`  

| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw-81-last-signature`](/contracts/cw81-last-signature/)       | Contract owner stores a exprirable signaturen and verifications happens against it |
| [`cw-81-pubkey`](/contracts/cw81-pubkey/)                       | Using secp256k1 public key provided by contract creator and verifying using ecdsa  |
| [`cw-81-sn-ks`](/contracts/cw81-sn-ks/)                         | SecretWasm based contract that uses a secp256k1 private key for signature generation and verification |

