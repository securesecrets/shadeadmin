use cosmwasm_std::{QuerierWrapper, StdError, StdResult, Addr};
use shade_protocol::{utils::Query, Contract};

use crate::admin::{QueryMsg, ValidateAdminPermissionResponse};

/// Returns an error if the user does not have the passed permission.
pub fn validate_permission(
    querier: &QuerierWrapper,
    permission: &str,
    user: &(impl Into<String> + Clone),
    admin_auth: &(impl Into<Contract> + Clone),
) -> StdResult<()> {
    let admin_resp: StdResult<ValidateAdminPermissionResponse> =
        QueryMsg::ValidateAdminPermission {
            permission: permission.to_string(),
            user: user.clone().into(),
        }
        .query(querier, admin_auth);

    match admin_resp {
        Ok(resp) => match resp.has_permission {
            true => Ok(()),
            false => Err(StdError::generic_err("Unexpected response.")),
        },
        Err(err) => Err(err),
    }
}