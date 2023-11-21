# CW Extra
Experimental CosmWasm based contracts, packages and protocols 



## Standards



| Name                                      | Description                                                           |  Examples      |
| ----------------------------------------- | --------------------------------------------------------------------- | -------------- |
| [`cw81`](./packages/cw81/)                | Signature verification for smart contracts (inspired by ERC-1271)     | [link](#cw81) |
| [`cw82`](./packages/cw82/)                | Minimal smart contract based abstract account                         | [link](#cw82) |
| [`cw83`](./packages/cw83/)                | Registry for smart contract based accounts (inspired by ERC-6551)     | [link](#cw83) |


## Contracts

Example contracts showcasing usage of the proposed protocols and standards

### CW81
| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw-81-last-signature`](./contracts/cw81-last-signature/)       | Contract owner stores an exprirable signature and checks a queried one is equal to it |
| [`cw-81-pubkey`](./contracts/cw81-pubkey/)                       | Using secp256k1 public key provided by contract creator and verifying using ecdsa  |
| [`cw-81-sn-ks`](./contracts/cw81-sn-ks/)                         | SecretWasm based contract using a secp256k1 private key for signature generation and verification |

### CW82
| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw82-key-account`](/contracts/cw82-key-account/)               | Signatures are verified against secp256k1 public key and all executable cosmos message must be signed by a corresponding private key |
| [`cw82-token-account`](/contracts/cw82-token-account/)           | Only an NFT owner can execute some cosmos messages. Signature are checked against the stored public key through [direct sign](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-036-arbitrary-signature.md)    |
| [`cw82-sn-sym`](https://github.com/MegaRockLabs/cw-extra/tree/secret-network/contracts/cw82-sn-sym)      | Secret Network specifc contract that only allow cosmos messages that had been encrypted by a secret (symmetric) key provided to the contract by instantiator. Signatures must be coming from a separate key generated inside the contract  to be valid |


### CW83
| Contract                                                         | Description                                                  |
| ---------------------------------------------------------------- | ------------------------------------------------------------ |
| [`cw83-tba-registry`](contracts/cw83-tba-registry)               | A Registry of token (NFT) bound accounts                     |


<br/>

---


### Purpose
This is playground repository for MegaRock to test out new ideas that are standartisable. All the projects will be of an experimental nature with low-maintanace and support. The projects here shouldn't be used for production before being peer-review and explicitly green-lighted by the team. (in case where the usage is licensed )

### Disclaimer
As part of our mission of promoting public goods and open source culture we are planning to be contributing the standards that gain adoption into the original [repository](https://github.com/CosmWasm/cw-plus) maintained by the official team working on CosmWasm. Altenatively the team might choose to maintain a standard ourselves and release it under permissive open sourse license. 

Until then everything in this repository is treated as source available only unless a project comes with a separate License file. The exact license for the whole repository is to be specified


