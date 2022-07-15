use shade_protocol::{
    cosmwasm_schema::{cw_serde, QueryResponses},
    c_std::{Addr, Deps, StdError, StdResult},
    thiserror::Error,
    utils::{InstantiateCallback, ExecuteCallback, Query}, Contract,
};

pub type AdminAuthResult<T> = core::result::Result<T, AdminAuthError>;

#[cw_serde]
pub struct InstantiateMsg {
    pub super_admin: Option<String>,
}

impl InstantiateCallback for InstantiateMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateRegistry { action: RegistryAction },
    UpdateRegistryBulk { actions: Vec<RegistryAction> },
    TransferSuper { new_super: String },
    SelfDestruct {},
    ToggleStatus { new_status: bool },
}

#[cw_serde]
pub enum RegistryAction {
    RegisterAdmin { admin: String },
    AddContract { contract: String },
    RemoveContract { contract: String },
    GrantAccess { contract: String, admin: String },
    RevokeAccess { contract: String, admin: String },
    DeleteAdmin { admin: String },
}

impl ExecuteCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
    #[returns(ContractsResponse)]
    GetContracts {},
    #[returns(AdminsResponse)]
    GetAdmins {},
    #[returns(PermissionsResponse)]
    GetPermissions { user: String },
    #[returns(ValidateAdminPermissionResponse)]
    ValidateAdminPermission { contract: String, user: String },
}

impl Query for QueryMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
pub struct ConfigResponse {
    pub super_admin: Addr,
    pub active: bool,
}

#[cw_serde]
pub struct PermissionsResponse {
    pub contracts: Vec<Addr>,
}

#[cw_serde]
pub struct ContractsResponse {
    pub contracts: Vec<Addr>,
}

#[cw_serde]
pub struct AdminsResponse {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub struct ValidateAdminPermissionResponse {
    pub is_admin: bool,
}

#[derive(Error, Debug, PartialEq)]
pub enum AdminAuthError {
    #[error("{0}")]
    // let thiserror implement From<StdError> for you
    Std(#[from] StdError),
    // this is whatever we want
    #[error("Registry error: user has not been registered as an admin.")]
    UnregisteredAdmin { user: Addr },
    #[error("Registry error: contract has not been registered.")]
    UnregisteredContract { unregistered_contract: Addr },
    #[error("Permission denied: user is not an admin for this contract.")]
    UnauthorizedAdmin { contract: Addr },
    #[error("Permission denied: user is not the authorized super admin.")]
    UnauthorizedSuper { expected_super_admin: Addr },
}

pub fn validate_admin(
    deps: Deps,
    contract_address: String,
    admin_address: String,
    shd_admin: Contract,
) -> StdResult<bool> {
    let msg = QueryMsg::ValidateAdminPermission {
        contract: contract_address,
        user: admin_address,
    };
    let admin_response: ValidateAdminPermissionResponse =
        deps.querier
            .query_wasm_smart(shd_admin.code_hash, shd_admin.address, &msg)?;
    Ok(admin_response.is_admin)
}
