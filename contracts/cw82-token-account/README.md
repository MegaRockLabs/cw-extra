# CW82: Token Account

A Smart Contract controlled by an owner of an non-fungible token. Relies on [CW83-TBA-Registry](/contracts/cw83-tba-registry) for verifing the ownership and perfoming upgrades. 

A contract must have a public key for signatire verification. Signature are checked against the stored public key through [direct sign](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-036-arbitrary-signature.md) method
