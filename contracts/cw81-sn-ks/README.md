# CW81: Secret Network Key Simple

A simple contract using private key for both signature generation and verification. Can only work inside a secure enclave where users can't read the private key. Not the most logical example showcasing CW81 since technically signatures come from the contract and verified by as well. Useful for showcasing loyalty to the interface for unifying expiereince of the dapps

Note: Shouldn't be used for sensitive data. Good idea to add a method for reseting the signing key
