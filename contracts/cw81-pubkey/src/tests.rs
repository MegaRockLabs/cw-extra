#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use types::wasm::testing::{message_info, mock_dependencies, mock_env};
    use cosmwasm_std::{
        from_json, to_json_binary, Binary
    };

    use k256::{
        ecdsa::{
            signature::DigestSigner,
            SigningKey, VerifyingKey, Signature
        },
        elliptic_curve::rand_core::OsRng
    };

    use sha2::{
        Sha256, 
        digest::{Update, Digest}
    };

    use cw81::ValidSignatureResponse;
    use crate::{contract::{instantiate, query}, msg::{InstantiateMsg, QueryMsg}};


    #[test]
    fn valid_signature_test() {

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = message_info(&deps.api.addr_make("creator"), &[]);

     
        let secret_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&secret_key);

        let another_key = SigningKey::random(&mut OsRng);

        // user store public key
        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {
            pubkey: public_key.to_encoded_point(false).as_bytes().into()
        }).unwrap();

        // dapp asks user to sign message
        let data : Binary = to_json_binary("message").unwrap();
        let data_digest = Sha256::new().chain(&data);

        // user signs message
        let signature: Signature = secret_key.sign_digest(data_digest.clone());

        // and gives signature to the dapp
        let query_msg = QueryMsg::ValidSignature { 
            signature: signature.to_bytes().as_slice().into(), 
            data: data.clone(), 
            payload: None 
        };

        // dapp verifies signature from the contract
        let query_res = query(deps.as_ref(), env.clone(), query_msg.clone()).unwrap();
        let res : ValidSignatureResponse = from_json(&query_res).unwrap();
        assert_eq!(res.is_valid, true);


        // if users has another key the signature is wrong
        let signature: Signature = another_key.sign_digest(data_digest);


        let another_msg = QueryMsg::ValidSignature { 
            signature: signature.to_bytes().as_slice().into(), 
            data, 
            payload: None 
        };      
        let query_res = query(deps.as_ref(), env.clone(), another_msg).unwrap();
        let res : ValidSignatureResponse = from_json(&query_res).unwrap();
        assert_eq!(res.is_valid, false);
    }

}