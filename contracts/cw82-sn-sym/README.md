# CW82: Secret Network Symmetric Key

Secret Network specifc contract implementing CW82 allowing only cosmos messages that had been encrypted by a secret key provided to the contract by instantiator. 

Normal cosmos messages aren't supported by default and must be given as a payload to the custom cosmos message defined in the contract
Signatures are generated using a separate key generated inside the contract

Note: Doesn't protect against replay attacks and isn't suitabale for production
