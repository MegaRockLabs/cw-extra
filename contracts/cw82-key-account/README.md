# CW82: Key Account
An abstract account using secp256k1 public key provided by contract creator for checking whether messages are executable and for verifying the signatures using ecdsa schema. Similar to how things works normally off-chain

Normal cosmos messages aren't supported by default and must be sent as a payload alongside the signature inside the defined custom message 

Note: Doesn't protect against replay attacks and doesn't restrict on who can change the public key. Mot for production
