#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info}, 
        Binary, from_binary, to_binary
    };

    use cw23::ValidSignatureResponse;
    use crate::{contract::{instantiate, query}, msg::{InstantiateMsg, QueryMsg}};


    #[test]
    fn simple_test() {

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);


        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {}).unwrap();


        let data : Binary = to_binary("message").unwrap();


        let signature : Binary = from_binary(
            &query(deps.as_ref(), env.clone(), QueryMsg::Signature { 
                to_sign: data.clone()
            }).unwrap()
        ).unwrap();


        let query_msg = QueryMsg::ValidSignature { 
            signature, 
            data: data.clone(), 
            payload: None 
        };

        let query_res = query(deps.as_ref(), env.clone(), query_msg.clone()).unwrap();
        let res : ValidSignatureResponse = from_binary(&query_res).unwrap();
        assert_eq!(res.is_valid, true);


        // if users has another key the signature is wrong
        let signature: Binary = Binary::from("scammy signature".as_bytes());

        let another_msg = QueryMsg::ValidSignature { 
            signature,
            data, 
            payload: None 
        };      
        let query_res = query(deps.as_ref(), env.clone(), another_msg).unwrap();
        let res : ValidSignatureResponse = from_binary(&query_res).unwrap();
        assert_eq!(res.is_valid, false);
        
    }

}