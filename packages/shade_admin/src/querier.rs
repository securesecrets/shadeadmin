use cosmwasm_std::{QuerierWrapper, StdResult, StdError};
use shade_protocol::{Contract, utils::Query};

use crate::admin::{ValidateAdminPermissionResponse, QueryMsg};

/// Returns an error if the user does not have the passed permission.
pub fn validate_permission(
    querier: &QuerierWrapper,
    permission: &str,
    user: String,
    admin_auth: &Contract,
) -> StdResult<()> {
    let admin_resp: StdResult<ValidateAdminPermissionResponse> = QueryMsg::ValidateAdminPermission {
        permission: permission.to_string(),
        user,
    }.query(querier, admin_auth);

    match admin_resp {
        Ok(resp) => match resp.has_permission {
            true => Ok(()),
            false => Err(StdError::generic_err("Unexpected response.")),
        },
        Err(err) => Err(err),
    }
}
