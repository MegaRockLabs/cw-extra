# CW82: Secret Network Symmetric Key

Secret Network specifc contract implementing cw that only allow cosmos messages that were encrypted by a secret key provided to the contract by instantiator. 

Normal cosmos messages aren't supported by default and must be given as a payload to defined custom cosmos message
Signatures must be coming from a separate key generated inside the contract

Note: Doesn't protect against replay attacks and isn't suitabale for production
