use cosmwasm_std::{Addr, StdResult};
use shade_admin::{multi_test::{App}, admin::{ConfigResponse, QueryMsg, InstantiateMsg, ValidateAdminPermissionResponse, AdminAuthError}, core::cosmwasm_std};
use crate::{AdminAuth, MultiTestable};

#[test]
fn basic_admin_test() {
    let owner = Addr::unchecked("owner");
    let super_admin = Addr::unchecked("superadmin");
    let mut router = App::default();

    let msg = InstantiateMsg { super_admin: Some(super_admin.to_string()) };
    let mock_admin = AdminAuth::init(&mut router, owner, "admin_auth", &[], &msg);
    let admin_auth = AdminAuth::new(mock_admin);
    let resp: ConfigResponse = admin_auth.query(&router, &QueryMsg::GetConfig {}).unwrap();
    assert!(resp.active);
    assert_eq!(resp.super_admin, super_admin);

    let resp: StdResult<ValidateAdminPermissionResponse> = admin_auth.query(&router, &QueryMsg::ValidateAdminPermission { contract: "blah".to_string(), user: "owner".to_string() });
    assert!(resp.is_err());
    let err = resp.err().unwrap();
    assert!(err.to_string().contains("not been registered as an admin"));
}