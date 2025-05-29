use protos::{signed_execute_one, signed_query_one};
use saa_common::types::msgs::SignedDataMsg;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::from_json;


#[signed_execute_one]
#[cw_serde]
enum ExecuteMsg {
    Foo {  },
}


#[signed_query_one(ExecuteMsg)]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {}



#[signed_query_one(ExecuteMsg, SignedDataMsg)]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsgSigned {}



#[test]
fn check_cw84_msgs_one() {

     let exec = from_json::<ExecuteMsg>("{\"execute_signed\":{\"msg\":{\"foo\":{}},\"signed\":\"\"}}");
    assert!(exec.is_ok(), "Failed to parse ExecuteMsg: {:?}", exec.err());
    
    let query  = from_json::<QueryMsg>("{\"can_execute_signed\":{\"msg\":{\"foo\":{}},\"signed\":\"\"}}");
    assert!(query.is_ok(), "Failed to parse QueryMsg: {:?}", query.err());

    let query  = from_json::<QueryMsgSigned>(
        "{\"can_execute_signed\":{\"msg\":{\"foo\":{}},\"signed\":{\"data\":\"\",\"signature\":\"\"}}}"
    );
    assert!(query.is_ok(), "Failed to parse QueryMsgSigned: {:?}", query.err());
}