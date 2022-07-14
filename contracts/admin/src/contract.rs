use shade_admin::admin::{InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse, ContractsResponse, ValidateAdminPermissionResponse, AdminAuthResult, AdminAuthError, AdminsResponse, PermissionsResponse, RegistryAction};
use shade_admin::core::cosmwasm_std;
use shade_admin::storage::{Map, Item};
use cosmwasm_std::{
    to_binary, Addr, Env, Deps, DepsMut, Response, StdResult, Storage, MessageInfo, entry_point, QueryResponse
};

/// Maps user to contracts for which they have admin.
const PERMISSIONS: Map<&Addr, Vec<Addr>> = Map::new("permissions");
/// List of all contracts that can refer to this admin auth.
const CONTRACTS: Item<Vec<Addr>> = Item::new("contracts");
/// List of all admins.
const ADMINS: Item<Vec<Addr>> = Item::new("admins");
/// Super admin.
const SUPER: Item<Addr> = Item::new("super");
/// Whether or not this contract can be consumed.
const IS_ACTIVE: Item<bool> = Item::new("is_active");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let super_admin = msg.super_admin.unwrap_or_else(|| info.sender.to_string());
    let super_admin_addr = deps.api.addr_validate(super_admin.as_str())?;
    SUPER.save(deps.storage, &super_admin_addr)?;

    ADMINS.save(deps.storage, &Vec::new())?;
    CONTRACTS.save(deps.storage, &Vec::new())?;
    IS_ACTIVE.save(deps.storage, &true)?;

    let res = Response::new()
        .add_attribute("action", "initialized")
        .add_attribute("superadmin", &info.sender);
    Ok(res)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> AdminAuthResult<Response>  {
    // Only the super admin can execute anything on this contract.
    is_super(deps.storage, &info.sender)?;
    // Super admin is assumed to have been verified by this point.
    match msg {
        ExecuteMsg::UpdateRegistry { action } => try_update_registry(deps, action),
        ExecuteMsg::UpdateRegistryBulk { actions } => try_update_registry_bulk(deps, actions),
        ExecuteMsg::TransferSuper { new_super } => try_transfer_super(deps, new_super),
        ExecuteMsg::SelfDestruct {  } => try_self_destruct(deps),
        ExecuteMsg::ToggleStatus { new_status } => try_toggle_status(deps, new_status),
    }
}

fn try_update_registry(deps: DepsMut, action: RegistryAction) -> AdminAuthResult<Response> {
    Ok(Response::default())
}

fn try_update_registry_bulk(deps: DepsMut, actions: Vec<RegistryAction>) -> AdminAuthResult<Response> {
    Ok(Response::default())
}

fn try_transfer_super(deps: DepsMut, new_super: String) -> AdminAuthResult<Response> {
    Ok(Response::default())
}

fn try_self_destruct(deps: DepsMut) -> AdminAuthResult<Response> {

    // Clear permissions

    // Clear admins

    // Clear contracts

    Ok(Response::default())
}

fn try_toggle_status(deps: DepsMut, new_status: bool) -> AdminAuthResult<Response> {
    IS_ACTIVE.update(deps.storage, |_| -> StdResult<_> {
        Ok(new_status)
    })?;
    Ok(Response::default())
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> AdminAuthResult<QueryResponse> {
    match msg {
        QueryMsg::GetConfig { } => {
            Ok(to_binary(&ConfigResponse {
                super_admin: SUPER.load(deps.storage)?,
                active: IS_ACTIVE.load(deps.storage)?,
            })?)},
        QueryMsg::ValidateAdminPermission { contract, user } => 
            Ok(to_binary(&query_validate_permission(deps, contract, user)?)?),
        QueryMsg::GetContracts {  } => 
            Ok(to_binary(&ContractsResponse { 
                contracts: CONTRACTS.load(deps.storage)?
            })?),
        QueryMsg::GetAdmins {  } => Ok(to_binary(&AdminsResponse { 
            admins: ADMINS.load(deps.storage)?
        })?),
        QueryMsg::GetPermissions { user } => {
            let validated_user = deps.api.addr_validate(user.as_str())?;
            Ok(to_binary(&PermissionsResponse {
                contracts: PERMISSIONS.load(deps.storage, &validated_user)?
            })?)
        },
    }
}

fn is_super(storage: &dyn Storage, address: &Addr) -> AdminAuthResult<()> {
    let super_admin = SUPER.load(storage)?;
    if super_admin == *address {
        Ok(())
    } else {
        Err(AdminAuthError::UnauthorizedSuper { expected_super_admin: super_admin })
    }
}

fn query_validate_permission(deps: Deps, contract: String, user: String) -> AdminAuthResult<ValidateAdminPermissionResponse> {
    let valid_contract = deps.api.addr_validate(contract.as_str())?;
    let valid_user = deps.api.addr_validate(user.as_str())?;
	let super_admin = SUPER.load(deps.storage)?;

    let is_admin: bool;

    if valid_user == super_admin {
        is_admin = true;
    } else {
        let permissions = PERMISSIONS.may_load(deps.storage, &valid_user)?;
        match permissions {
            Some(permissions) => {
                if permissions.iter().any(|c| valid_user == *c) {
                    is_admin = true;
                } else {
                    return Err(AdminAuthError::UnauthorizedAdmin { contract: valid_contract });
                }
            },
            // If user has been registered, there should be an empty vector there.
            None => return Err(AdminAuthError::UnregisteredAdmin { user: valid_user }),
        }
    }
    Ok(ValidateAdminPermissionResponse { is_admin })
}