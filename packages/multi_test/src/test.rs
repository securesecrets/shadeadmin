use crate::multi::{AdminAuth};
use shade_admin::{
    admin::{
        ConfigResponse, InstantiateMsg, QueryMsg, ValidateAdminPermissionResponse, ExecuteMsg, RegistryAction, AdminAuthStatus, AdminAuthError,
    },
};
use shade_protocol::{c_std::{Addr, StdResult}, multi_test::App, utils::{InstantiateCallback, ExecuteCallback, Query, MultiTestable}};

#[test]
fn basic_admin_test() {
    let owner = Addr::unchecked("owner");
    let super_admin = Addr::unchecked("superadmin");
    let perm_1 = "ORACLE_ADMIN".to_string();
    let mut router = App::default();

    let mock_admin = InstantiateMsg {
        super_admin: Some(super_admin.to_string()),
    }.test_init(AdminAuth::default(), &mut router, owner.clone(), "admin_auth", &[]).unwrap();

    let resp: ConfigResponse = QueryMsg::GetConfig {  }.test_query(&mock_admin, &router).unwrap();    
    assert!(resp.status.eq(&AdminAuthStatus::Active));
    assert_eq!(resp.super_admin, super_admin);

    let resp: StdResult<ValidateAdminPermissionResponse> = QueryMsg::ValidateAdminPermission {
        permission: perm_1.clone(),
        user: "owner".to_string(),
    }.test_query(&mock_admin, &router);

    assert!(resp.is_err());
    let err = resp.err().unwrap();
    assert!(err.to_string().contains(&AdminAuthError::UnregisteredAdmin { user: owner.clone() }.to_string()));

    ExecuteMsg::ToggleStatus { new_status: AdminAuthStatus::Shutdown }.test_exec(&mock_admin, &mut router, super_admin.clone(), &[]).unwrap();

    let resp: ConfigResponse = QueryMsg::GetConfig {  }.test_query(&mock_admin, &router).unwrap();    
    assert!(resp.status.eq(&AdminAuthStatus::Shutdown));

    assert!(ExecuteMsg::UpdateRegistryBulk { actions: vec![
        RegistryAction::RegisterAdmin { user: owner.to_string() }, 
        RegistryAction::GrantAccess { permissions: vec![perm_1.clone()], user: owner.to_string() }, 
    ]}.test_exec(&mock_admin, &mut router, super_admin.clone(), &[]).is_err());

    ExecuteMsg::ToggleStatus { new_status: AdminAuthStatus::Maintenance }.test_exec(&mock_admin, &mut router, super_admin.clone(), &[]).unwrap();

    let resp: ConfigResponse = QueryMsg::GetConfig {  }.test_query(&mock_admin, &router).unwrap();    
    assert!(resp.status.eq(&AdminAuthStatus::Maintenance));

    ExecuteMsg::UpdateRegistryBulk { actions: vec![
        RegistryAction::RegisterAdmin { user: owner.to_string() }, 
        RegistryAction::GrantAccess { permissions: vec![perm_1.clone()], user: owner.to_string() }, 
    ]}.test_exec(&mock_admin, &mut router, super_admin.clone(), &[]).unwrap();

    let resp: StdResult<ValidateAdminPermissionResponse> = QueryMsg::ValidateAdminPermission { permission: perm_1.clone(), user: owner.to_string() }.test_query(&mock_admin, &router);

    assert!(resp.err().unwrap().to_string().contains(&AdminAuthError::IsUnderMaintenance.to_string()));

    ExecuteMsg::ToggleStatus { new_status: AdminAuthStatus::Active }.test_exec(&mock_admin, &mut router, super_admin.clone(), &[]).unwrap();

    let resp: StdResult<ValidateAdminPermissionResponse> = QueryMsg::ValidateAdminPermission { permission: perm_1.clone(), user: owner.to_string() }.test_query(&mock_admin, &router);
    assert!(resp.is_ok_and(|r| r.has_permission));

}
