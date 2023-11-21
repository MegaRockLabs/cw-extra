#[cfg(test)]
mod tests {
    use cosmwasm_std::{Binary, to_binary};
    use cosmwasm_crypto::{secp256k1_verify, ed25519_batch_verify};
    
    use k256::{
        ecdsa::{
            signature::DigestSigner,
            SigningKey, VerifyingKey, Signature
        },
        elliptic_curve::rand_core::OsRng
    };

    use ed25519_zebra::{
        SigningKey as Ed25519SigningKey,
        VerificationKey as Ed25519VerificationKey
    };

    use sha2::{
        Sha256, 
        digest::{Update, Digest}
    };

    use crate::msg::Cw81QueryMsg;

    
    const MSG: &str = "Testing String!";


    #[test]
    fn simple_secp256k1() {

        let data : Binary = to_binary(&MSG).unwrap();
        let data_digest = Sha256::new().chain(&data);

        let secret_key = SigningKey::random(&mut OsRng);
        let signature: Signature = secret_key.sign_digest(data_digest);

        let public_key = VerifyingKey::from(&secret_key);

        let msg = Cw81QueryMsg::ValidSignature { 
            data: data, 
            signature: signature.to_bytes().as_slice().into(), 
            payload: Some(public_key.to_encoded_point(false).as_bytes().into()) 
        };


        match msg {
            Cw81QueryMsg::ValidSignature { 
                data, 
                signature, 
                payload 
            } => {

                let hash = Sha256::new().chain(&data).finalize();
                let public_key = payload;

                assert!(
                    secp256k1_verify(
                        &hash,
                        &signature,
                        &public_key.unwrap()
                    ).unwrap()
                );

            },

            _ => { /* QueryMsg::ValidSignatures */ }
        }

        
    }

    #[test]
    fn batch_ed25519() {

        let msg : Binary = to_binary(&MSG).unwrap();
        let msg_digest = Sha256::new().chain(&msg).finalize();

        let secret_key = Ed25519SigningKey::new(OsRng);
        let pub_key_bytes : [u8; 32] = Ed25519VerificationKey::from(&secret_key).into();
        let pub_key_binary : Binary = pub_key_bytes.into();
        
        let signature = secret_key.sign(&msg_digest);
        let signature : Binary = signature.to_bytes().as_slice().into();
        
        let another_msg : Binary = to_binary("another msg").unwrap();
        let another_digest = Sha256::new().chain(&another_msg).finalize();
        let another_signature : Binary = secret_key.sign(&another_digest).to_bytes().as_slice().into();

        let data = vec![msg, another_msg];
        let signatures = vec![signature, another_signature];
   

        let msg = Cw81QueryMsg::ValidSignatures { 
            data, 
            signatures,
            payload: Some(pub_key_binary) 
        };


        match msg {
            Cw81QueryMsg::ValidSignatures { 
                data, 
                signatures, 
                payload 
            } => {

                let hashes : Vec<Vec<u8>> = data
                    .iter()
                    .map(|d|  Sha256::new().chain(&d).finalize().to_vec())
                    .collect();

                let hashes : Vec<&[u8]> = hashes
                    .iter()
                    .map(|h| h.as_slice())
                    .collect();

                let signatures : Vec<&[u8]> = signatures
                    .iter()
                    .map(|sig|sig.as_slice())
                    .collect();

                let public_key = payload.unwrap();

                let keys_count = hashes.len();
                let mut keys: Vec<&[u8]> = Vec::with_capacity(keys_count);

                for _ in 0..keys_count {
                    keys.push(public_key.as_slice());
                }

                assert!(
                    ed25519_batch_verify(
                        &hashes,
                        &signatures,
                        &keys
                    ).unwrap()
                );

            },

            _ => { /* QueryMsg::ValidSignature */ }
        }

        
    }


}