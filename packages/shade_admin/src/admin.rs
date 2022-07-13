
use serde::{Deserialize, Serialize};
use cosmwasm_std::{StdError, Addr, to_binary as _to_binary, Binary, Deps, StdResult};
use thiserror::Error;
//use secret_toolkit::utils::{HandleCallback, Query};

pub type AdminAuthResult<T> = core::result::Result<T, AdminAuthError>;

#[cfg(feature = "impl")]
pub fn to_binary<T>(data: &T) -> AdminAuthResult<Binary>
where
    T: Serialize + ?Sized,
{
	_to_binary(data).map_err(AdminAuthError::Std)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
	pub super_admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum ExecuteMsg {
	UpdateRegistry {
		action: RegistryAction,
	},
	UpdateRegistryBulk {
		actions: Vec<RegistryAction>,
	},
	TransferSuper {
		new_super: String,
	},
	Selfdestruct { },
	ToggleStatus {
		active: bool,
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum RegistryAction {
	RegisterAdmin {
		admin: String,
	},
	AddContract{
		contract: String
	},
	RemoveContract{
		contract: String
	},
	GrantAccess {
		contract: String,
		admin: String
	},
	RevokeAccess { 
		contract: String,
		admin: String
	},
	DeleteAdmin {
		admin: String
	},
}

// impl HandleCallback for ExecuteMsg {
//     const BLOCK_SIZE: usize = 256;
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum QueryMsg {
	GetConfig { },
	GetContracts { },
	GetAdmins { },
	GetPermissions { user: String },
	ValidateAdminPermission {
		contract: String,
		user: String
	},
}

// impl Query for QueryMsg {
//     const BLOCK_SIZE: usize = 256;
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
	pub super_admin: Addr,
	pub active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PermissionsResponse {
	pub contracts: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractsResponse {
	pub contracts: Vec<Addr>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AdminsResponse {
	pub admins: Vec<Addr>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ValidateAdminPermissionResponse {
	pub is_admin: bool
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

/// Delete this when we implement the updated shade protocol package
#[derive(Serialize, Deserialize)]
pub struct Contract {
	pub address: Addr,
	pub code_hash: String,
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
    let admin_response: ValidateAdminPermissionResponse = deps.querier.query_wasm_smart(shd_admin.code_hash, shd_admin.address, &msg)?;
    Ok(admin_response.is_admin)
}