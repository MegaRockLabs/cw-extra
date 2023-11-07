#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info}, 
        Binary, from_binary, to_binary, CosmosMsg, BankMsg, coins
    };

    use crate::{contract::{instantiate, query}, msg::{InstantiateMsg, QueryMsg, EncryptedMsg, CanExecuteResponse}};

    use k256::elliptic_curve::rand_core::OsRng;
    use ecies::{SecretKey, symmetric::sym_encrypt};


    #[test]
    fn can_execute_test() {

        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let secret_key = SecretKey::random(&mut OsRng);
        let key_bytes = secret_key.serialize();


        instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {
            secret_key: key_bytes.into()
        }).unwrap();

        
        // normal bank message
        let msg : CosmosMsg<EncryptedMsg> = BankMsg::Send { 
            to_address: "test".into(), 
            amount: coins(1, "test") 
        }.into();


        let res : CanExecuteResponse = from_binary(&query(deps.as_ref(), env.clone(), 
            QueryMsg::CanExecute { 
                sender: "test".into(),
                msg: msg.clone()
            }
        ).unwrap()).unwrap();

        // only supporting encrypted messages
        assert_eq!(res.can_execute, false);

        let encrypted_msg : Binary =  sym_encrypt(
            &key_bytes, 
            &to_binary(&msg).unwrap(),
            OsRng
        ).unwrap().into();


        let msg : CosmosMsg<EncryptedMsg> = EncryptedMsg {
            encrypted_msg
        }.into();


        let res : CanExecuteResponse = from_binary(&query(deps.as_ref(), env.clone(), 
            QueryMsg::CanExecute { 
                sender: "test".into(),
                msg
            }
        ).unwrap()).unwrap();

        assert_eq!(res.can_execute, true);
        
    }

}