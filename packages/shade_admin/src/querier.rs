use cosmwasm_std::{QuerierWrapper, StdError, StdResult, Addr};
use shade_protocol::{utils::Query, Contract};

use crate::admin::{QueryMsg, ValidateAdminPermissionResponse};

/// Returns an error if the user does not have the passed permission.
pub fn validate_permission<T: Into<String> + Clone>(
    querier: &QuerierWrapper,
    permission: &T,
    user: &Addr,
    admin_auth: &Contract,
) -> StdResult<()> {
    let admin_resp: StdResult<ValidateAdminPermissionResponse> =
        QueryMsg::ValidateAdminPermission {
            permission: permission.clone().into(),
            user: user.clone().to_string(),
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
