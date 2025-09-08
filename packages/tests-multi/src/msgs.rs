// use saa_common::types::signed::SignedDataMsg;
use cosmwasm_schema::{cw_serde, QueryResponses};
use protos::{signed_execute_multi, signed_query_multi};
use cosmwasm_std::from_json;
use types::SignedDataMsg;



#[signed_execute_multi]
#[cw_serde]
enum ExecuteMsg {
    Foo {  },
}

#[signed_execute_multi(ExecuteMsg, SignedDataMsg)]
#[cw_serde]
enum ExecuteMsgCustom {
    Bar {  },
}


#[signed_query_multi(ExecuteMsg)]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsg {}


#[signed_query_multi(ExecuteMsg, SignedDataMsg)]
#[cw_serde]
#[derive(QueryResponses)]
enum QueryMsgCustom {}



#[test]
fn check_cw84_msgs_multi() {

    let exec = from_json::<ExecuteMsg>("{\"execute_signed\":{\"msgs\":[{\"foo\":{}}],\"signed\":\"\"}}");
    assert!(exec.is_ok(), "Failed to parse ExecuteMsg: {:?}", exec.err());

    let exec = from_json::<ExecuteMsg>("{\"execute_native\":{\"msgs\":[{\"bar\":{}}]}");
    // native action aren't injected with the default Self type
    assert!(exec.is_err());

    let exec = from_json::<ExecuteMsgCustom>(
        "{\"execute_signed\":{\"msgs\":[{\"bar\":{}}],\"signed\":{\"data\":\"\",\"signature\":\"\"}}}"
    );
    // overriden action doesn't have bar, only foo
    assert!(exec.is_err());

    let exec = from_json::<ExecuteMsgCustom>(
        "{\"execute_signed\":{\"msgs\":[{\"foo\":{}}],\"signed\":{\"data\":\"\",\"signature\":\"\"}}}"
    );
    assert!(exec.is_ok(), "Failed to parse ExecuteMsgCustom: {:?}", exec.err());

    let exec = from_json::<ExecuteMsgCustom>("{\"execute_native\":{\"msgs\":[{\"foo\":{}}]}}");
    assert!(exec.is_ok(), "Failed to parse ExecuteMsgCustom: {:?}", exec.err());
    
    let query  = from_json::<QueryMsg>("{\"can_execute_signed\":{\"msgs\":[{\"foo\":{}}],\"signed\":\"\"}}");
    assert!(query.is_ok(), "Failed to parse QueryMsg: {:?}", query.err());

    let query  = from_json::<QueryMsgCustom>(
        "{\"can_execute_signed\":{\"msgs\":[{\"foo\":{}}],\"signed\":{\"data\":\"\",\"signature\":\"\"}}}"
    );
    assert!(query.is_ok(), "Failed to parse QueryMsgSigned: {:?}", query.err());
}