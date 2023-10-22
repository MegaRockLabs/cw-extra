# CW81: Secret Network Key Simple

A simple contract using private key for both signature genration and verification. Can only work inside a secure enclave where external users can't read the private key. Not the most logical example showcasing CW81 since technically signature comes from the contract and it will be verified by the it. Advantage is that the API stays the same

Note: Shouldn't be used for sensitive data. One can add a method for reseting the private key
