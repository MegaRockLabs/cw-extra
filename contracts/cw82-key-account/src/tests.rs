#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use types::wasm::{
        from_json, testing::{
            message_info, mock_dependencies, mock_env
        }
    };
    use cosmwasm_std::{BankMsg, CosmosMsg, Binary, to_json_binary, coins};

    use cw82::{CanExecuteResponse, ValidSignatureResponse};
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

    use crate::{
        contract::{instantiate, query}, 
        msg::{InstantiateMsg, SignedMsg, QueryMsg}
    };


    #[test]
    fn can_execute_test() {

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = message_info(&deps.api.addr_make("creator"), &[]);

        let secret_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&secret_key);

        // user store public key
        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {
            pub_key: public_key.to_encoded_point(false).as_bytes().into()
        }).unwrap();


        let bank =  BankMsg::Send { 
            to_address: "test".into(), 
            amount: coins(1, "test") 
        };


        let res : CanExecuteResponse = from_json(&query(deps.as_ref(), env.clone(), 
            QueryMsg::CanExecute { 
                sender: "test".into(),
                msg: bank.clone().into()
            }
        ).unwrap()).unwrap();

        // only supporting signed messages
        assert_eq!(res.can_execute, false);


        let signed_hash: Signature = secret_key.sign_digest(
            Sha256::new()
            .chain(&to_json_binary(&CosmosMsg::<SignedMsg>::Bank(bank.clone())).unwrap())
        );

        let msg =  types::wasm::CosmosMsg::Bank(
            types::wasm::BankMsg::Send { 
                to_address: "test".into(), 
                amount: types::wasm::coins(1, "test")
            }
        );

        let msg : CosmosMsg<SignedMsg> = CosmosMsg::Custom(SignedMsg {
            signed_hash: signed_hash.to_bytes().as_slice().into(),
            msg
        });


        let res : CanExecuteResponse = from_json(&query(deps.as_ref(), env.clone(), 
            QueryMsg::CanExecute { 
                sender: "test".into(),
                msg
            }
        ).unwrap()).unwrap();

        assert_eq!(res.can_execute, true);
    }


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
            pub_key: public_key.to_encoded_point(false).as_bytes().into()
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