use cosmwasm_std::{
    entry_point, to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
    StdResult, Storage, Api,
};
use shade_admin::admin::{
    AdminAuthError, AdminAuthResult, AdminsResponse, ConfigResponse, ExecuteMsg,
    InstantiateMsg, PermissionsResponse, QueryMsg, RegistryAction, ValidateAdminPermissionResponse,
};
use shade_admin::storage::{Item, Map};

/// Maps user to contracts for which they have user.
const PERMISSIONS: Map<&Addr, Vec<Addr>> = Map::new("permissions");
/// List of all admins.
const ADMINS: Item<Vec<Addr>> = Item::new("admins");
/// Super user.
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
) -> AdminAuthResult<Response> {
    // Only the super user can execute anything on this contract.
    is_super(deps.storage, &info.sender)?;
    // Super user is assumed to have been verified by this point.
    match msg {
        ExecuteMsg::UpdateRegistry { action } => resolve_registry_action(deps.storage, deps.api, action),
        ExecuteMsg::UpdateRegistryBulk { actions } => try_update_registry_bulk(deps, actions),
        ExecuteMsg::TransferSuper { new_super } => try_transfer_super(deps, new_super),
        ExecuteMsg::SelfDestruct {} => try_self_destruct(deps),
        ExecuteMsg::ToggleStatus { new_status } => try_toggle_status(deps, new_status),
    }
}

fn resolve_registry_action(store: &mut dyn Storage, api: &dyn Api, action: RegistryAction) -> AdminAuthResult<Response> {
    match action {
        RegistryAction::RegisterAdmin { user } => register_admin(store, api, user),
        RegistryAction::GrantAccess { contracts, user } => grant_access(store, api, contracts, user),
        RegistryAction::RevokeAccess { contracts, user } => revoke_access(store, api, contracts, user),
        RegistryAction::DeleteAdmin { user } => delete_admin(store, api,  user),
    }?;
    Ok(Response::default())
}

fn try_update_registry_bulk(
    deps: DepsMut,
    actions: Vec<RegistryAction>,
) -> AdminAuthResult<Response> {
    for action in actions {
        resolve_registry_action(deps.storage, deps.api, action)?;
    };
    Ok(Response::default())
}

fn register_admin(store: &mut dyn Storage, api: &dyn Api, user: String) -> AdminAuthResult<()> {
    let user_addr = api.addr_validate(user.as_str())?;
    let mut admins = ADMINS.load(store)?;
    if !admins.contains(&user_addr) {
        // Create an empty permissions for them and add their address to the registered array.
        admins.push(user_addr.clone());
        PERMISSIONS.save(store, &user_addr, &vec![])?;
        ADMINS.save(store, &admins)?;
    };
    Ok(())
}

fn delete_admin(store: &mut dyn Storage, api: &dyn Api, user: String) -> AdminAuthResult<()> {
    let user_addr = api.addr_validate(user.as_str())?;
    let mut admins = ADMINS.load(store)?;
    if admins.contains(&user_addr) {
        // Delete admin from list.
        admins.retain(|x| x.ne(&user_addr));
        // Clear their permissions.
        PERMISSIONS.save(store, &user_addr, &vec![])?;
        ADMINS.save(store, &admins)?;
    };
    Ok(())
}

fn verify_registered(store: &dyn Storage, user: &Addr) -> AdminAuthResult<()> {
    if !ADMINS.load(store)?.contains(user) {
        return Err(AdminAuthError::UnregisteredAdmin { user: user.clone() });
    }
    Ok(())
}

fn validate_addresses(api: &dyn Api, contracts: Vec<String>) -> AdminAuthResult<Vec<Addr>> {
    let mut validated_addresses = vec![];
    for contract in contracts {
        let addr = api.addr_validate(contract.as_str())?;
        validated_addresses.push(addr);
    }
    Ok(validated_addresses)
}

fn grant_access(store: &mut dyn Storage, api: &dyn Api, contracts: Vec<String>, user: String) -> AdminAuthResult<()> {
    let mut contracts = validate_addresses(api, contracts)?;
    let user = api.addr_validate(user.as_str())?;
    verify_registered(store, &user)?;
    PERMISSIONS.update(store, &user, |old_perms| -> AdminAuthResult<_> {
        match old_perms {
            Some(mut old_perms) => {
                contracts.retain(|c| !old_perms.contains(c));
                old_perms.append(&mut contracts);
                Ok(old_perms)
            },
            None => Err(AdminAuthError::NoPermissions {  }),
        }
    })?;
    Ok(())
}

fn revoke_access(store: &mut dyn Storage, api: &dyn Api, contracts: Vec<String>, user: String) -> AdminAuthResult<()> {
    let contracts = validate_addresses(api, contracts)?;
    let user = api.addr_validate(user.as_str())?;
    verify_registered(store, &user)?;
    PERMISSIONS.update(store, &user, |old_perms| -> AdminAuthResult<_> {
        match old_perms {
            Some(mut old_perms) => {
                old_perms.retain(|c| !contracts.contains(c));
                Ok(old_perms)
            },
            None => Err(AdminAuthError::NoPermissions {  }),
        }
    })?;
    Ok(())
}

fn try_transfer_super(deps: DepsMut, new_super: String) -> AdminAuthResult<Response> {
    let valid_super = deps.api.addr_validate(new_super.as_str())?;
    // If you're trying to transfer the super permissions to someone who hasn't been registered as an admin,
    // it won't work. This is a safeguard.
    if !ADMINS.load(deps.storage)?.contains(&valid_super) {
        return Err(AdminAuthError::UnregisteredAdmin { user: valid_super });
    } else {
        // Update the super and remove them from the admin list.
        SUPER.save(deps.storage, &valid_super)?;
        delete_admin(deps.storage, deps.api, new_super)?;
    }
    Ok(Response::default())
}

fn try_self_destruct(deps: DepsMut) -> AdminAuthResult<Response> {
    // Clear permissions
    let admins = ADMINS.load(deps.storage)?;
    admins.iter().for_each(|admin| {
        PERMISSIONS.remove(deps.storage, admin)
    });
    // Clear admins
    ADMINS.save(deps.storage, &vec![])?;
    // Disable contract
    IS_ACTIVE.save(deps.storage, &false)?;
    Ok(Response::default())
}

fn try_toggle_status(deps: DepsMut, new_status: bool) -> AdminAuthResult<Response> {
    IS_ACTIVE.update(deps.storage, |_| -> StdResult<_> { Ok(new_status) })?;
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> AdminAuthResult<QueryResponse> {
    match msg {
        QueryMsg::GetConfig {} => Ok(to_binary(&ConfigResponse {
            super_admin: SUPER.load(deps.storage)?,
            active: IS_ACTIVE.load(deps.storage)?,
        })?),
        QueryMsg::ValidateAdminPermission { contract, user } => Ok(to_binary(
            &query_validate_permission(deps, contract, user)?,
        )?),
        QueryMsg::GetAdmins {} => Ok(to_binary(&AdminsResponse {
            admins: ADMINS.load(deps.storage)?,
        })?),
        QueryMsg::GetPermissions { user } => {
            let validated_user = deps.api.addr_validate(user.as_str())?;
            Ok(to_binary(&PermissionsResponse {
                contracts: PERMISSIONS.load(deps.storage, &validated_user)?,
            })?)
        }
    }
}

fn is_super(storage: &dyn Storage, address: &Addr) -> AdminAuthResult<()> {
    let super_admin = SUPER.load(storage)?;
    if super_admin == *address {
        Ok(())
    } else {
        Err(AdminAuthError::UnauthorizedSuper {
            expected_super_admin: super_admin,
        })
    }
}

fn query_validate_permission(
    deps: Deps,
    contract: String,
    user: String,
) -> AdminAuthResult<ValidateAdminPermissionResponse> {
    let valid_contract = deps.api.addr_validate(contract.as_str())?;
    let valid_user = deps.api.addr_validate(user.as_str())?;
    let super_admin = SUPER.load(deps.storage)?;

    let is_admin: bool;

    // Super user has perms for every contract. The contracts don't need to be whitelisted ones, as long
    // as they implement user auth. We do this because we assume that the super user is secure (like a 
    // multi-sig) so it would be a hassle to whitelist every contract we want them to be able to use.
    if valid_user == super_admin {
        is_admin = true;
    } else {
        let permissions = PERMISSIONS.may_load(deps.storage, &valid_user)?;
        match permissions {
            Some(permissions) => {
                if permissions.iter().any(|c| valid_user == *c) {
                    is_admin = true;
                } else {
                    return Err(AdminAuthError::UnauthorizedAdmin {
                        contract: valid_contract,
                    });
                }
            }
            // If user has been registered, there should be an empty vector there.
            None => return Err(AdminAuthError::UnregisteredAdmin { user: valid_user }),
        }
    }
    Ok(ValidateAdminPermissionResponse { is_admin })
}
