use cosmwasm_std::QuerierWrapper;
use shade_protocol::{
    cosmwasm_schema::{cw_serde, QueryResponses},
    c_std::{Addr, StdError, StdResult},
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
    RegisterAdmin { user: String },
    GrantAccess { contracts: Vec<String>, user: String },
    RevokeAccess { contracts: Vec<String>, user: String },
    DeleteAdmin { user: String },
}

impl ExecuteCallback for ExecuteMsg {
    const BLOCK_SIZE: usize = 256;
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
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
    #[error("Permission denied: user is not an user for this contract.")]
    UnauthorizedAdmin { contract: Addr },
    #[error("Permission denied: user is not the authorized super admin.")]
    UnauthorizedSuper { expected_super_admin: Addr },
    #[error("Registry error: there are no permissions for this admin.")]
    NoPermissions { }
}

/// Returns an error if the user does not have admin privileges for the contract in question.
pub fn validate_admin(
    querier: &QuerierWrapper,
    contract: String,
    user: String,
    admin_auth: &Contract,
) -> StdResult<()> {
    let admin_resp: StdResult<ValidateAdminPermissionResponse> = QueryMsg::ValidateAdminPermission {
        contract,
        user,
    }.query(querier, admin_auth);

    match admin_resp {
        Ok(resp) => match resp.is_admin {
            true => Ok(()),
            false => Err(StdError::generic_err("Unexpected response.")),
        },
        Err(err) => Err(err),
    }
}
