#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use types::wasm::{testing::{mock_info, mock_dependencies, mock_env}};
    use cosmwasm_std::{from_json, Binary};
    use cw81::ValidSignatureResponse;

    use crate::{contract::{instantiate, execute, query}, msg::{InstantiateMsg, ExecureMsg, QueryMsg}};


    #[test]
    fn simple_test() {

        let mut deps = mock_dependencies();
        let mut env = mock_env();
        let info = mock_info("creator", &[]);


        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {}).unwrap();

        let signature = Binary::from("signature".as_bytes());

        let msg = ExecureMsg::SaveSignature { 
            signature: signature.clone().0.into(), 
            expiration: None 
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();



        let query_msg = QueryMsg::ValidSignature { 
            signature, data: Binary::default(), payload: None 
        };

        let query_res = query(deps.as_ref(), env.clone(), query_msg.clone()).unwrap();
        let res : ValidSignatureResponse = from_json(&query_res).unwrap();
        assert_eq!(res.is_valid, true);


        let another_msg = QueryMsg::ValidSignature { 
            signature: Binary::from("another".as_bytes()), data: Binary::default(), payload: None 
        };        
        let query_res = query(deps.as_ref(), env.clone(), another_msg).unwrap();
        let res : ValidSignatureResponse = from_json(&query_res).unwrap();
        assert_eq!(res.is_valid, false);


        env.block.height += 101;
        let query_res = query(deps.as_ref(), env.clone(), query_msg).unwrap();
        let res : ValidSignatureResponse = from_json(&query_res).unwrap();
        assert_eq!(res.is_valid, false);

    }

}