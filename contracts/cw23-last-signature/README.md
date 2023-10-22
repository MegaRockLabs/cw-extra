# CW23-Last-Signature

An owner can store a signature inside storage of the contract and set an expiration data. By default it's 100 blocks if not provided.
User can specify `Expiration::Never {}` is he want it to never expire.

Signature verification logic first check whether the stored signature is expired and verify that the provided in a query is equal to the one in the storage

Note: Need to add restrictions to who can change the signature
