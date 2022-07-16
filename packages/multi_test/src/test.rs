use crate::{AdminAuth};
use cosmwasm_std::{Addr, StdResult};
use shade_admin::{
    admin::{
        ConfigResponse, InstantiateMsg, QueryMsg, ValidateAdminPermissionResponse, ExecuteMsg,
    },
    core::cosmwasm_std,
};
use shade_protocol::{multi_test::App, utils::{InstantiateCallback, ExecuteCallback, Query, MultiTestable}};

#[test]
fn basic_admin_test() {
    let owner = Addr::unchecked("owner");
    let super_admin = Addr::unchecked("superadmin");
    let mut router = App::default();

    let mock_admin = InstantiateMsg {
        super_admin: Some(super_admin.to_string()),
    }.test_init(AdminAuth::default(), &mut router, owner, "admin_auth", &[]).unwrap();

    let resp: ConfigResponse = QueryMsg::GetConfig {  }.test_query(&mock_admin, &router).unwrap();    
    assert!(resp.active);
    assert_eq!(resp.super_admin, super_admin);

    let resp: StdResult<ValidateAdminPermissionResponse> = QueryMsg::ValidateAdminPermission {
        contract: "blah".to_string(),
        user: "owner".to_string(),
    }.test_query(&mock_admin, &router);

    assert!(resp.is_err());
    let err = resp.err().unwrap();
    //print!("{}", err);
    assert!(err.to_string().contains("not been registered as an admin"));

    ExecuteMsg::ToggleStatus { new_status: false }.test_exec(&mock_admin, &mut router, super_admin, &[]).unwrap();

    let resp: ConfigResponse = QueryMsg::GetConfig {  }.test_query(&mock_admin, &router).unwrap();    
    assert!(!resp.active);
}
