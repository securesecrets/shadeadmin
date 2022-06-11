use crate::constants::*;
use secretcli::{cli_types::NetContract, secretcli::query_contract};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use shade_admin::admin::{
    InitMsg, HandleMsg, QueryMsg, SuperAdminResponse,
    ContractsResponse, AuthorizedUsersResponse, ValidateAuthorityResponse
};

use super::{GasLog, TestableContract};

#[derive(Serialize, Deserialize)]
pub struct AdminAuthContract {
    pub info: NetContract,
}

impl TestableContract for AdminAuthContract {
    fn get_info(&self) -> &NetContract {
        &self.info
    }
    fn get_file() -> &'static str {
        ADMIN_AUTH_FILE
    }
}

impl AdminAuthContract {
    pub fn new(
        msg: &InitMsg,
        account_key: Option<&str>,
        name: Option<&str>,
    ) -> Result<Self> {
        let info = Self::wrap_init(msg, account_key, name)?;
        Ok(AdminAuthContract { info })
    }

    pub fn query_super_admins(&self) -> Result<SuperAdminResponse> {
        query_contract(self.get_info(), QueryMsg::GetSuperAdmins { })
    }

    pub fn query_contracts(&self) -> Result<ContractsResponse> {
        query_contract(self.get_info(), QueryMsg::GetContracts { })
    }

    pub fn query_authorized_users(&self, contract_hash: String) -> Result<AuthorizedUsersResponse> {
        query_contract(self.get_info(), QueryMsg::GetAuthorizedUsers { contract_hash })
    }

    pub fn query_validate_auth(&self, contract_hash: String, admin_address: String) -> Result<ValidateAuthorityResponse> {
        query_contract(self.get_info(), QueryMsg::ValidateAuthority { contract_hash, admin_address })
    }

    pub fn add_contract(&self, contract_hash: String, sender_key: Option<&str>) -> Result<GasLog> {
        let msg = HandleMsg::AddContract { contract_hash };
        self.wrap_handle(&msg, sender_key)
    }

    pub fn remove_contract(&self, contract_hash: String, sender_key: Option<&str>) -> Result<GasLog> {
        let msg = HandleMsg::RemoveContract { contract_hash };
        self.wrap_handle(&msg, sender_key)
    }

    pub fn add_authorization(&self, contract_hash: String, admin_address: String, sender_key: Option<&str>) -> Result<GasLog> {
        let msg = HandleMsg::AddAuthorization { contract_hash, admin_address };
        self.wrap_handle(&msg, sender_key)
    }

    pub fn remove_authorization(&self, contract_hash: String, admin_address: String, sender_key: Option<&str>) -> Result<GasLog> {
        let msg = HandleMsg::RemoveAuthorization { contract_hash, admin_address };
        self.wrap_handle(&msg, sender_key)
    }

    pub fn add_super(&self, super_address: String, sender_key: Option<&str>) -> Result<GasLog> {
        let msg = HandleMsg::AddSuper { super_address };
        self.wrap_handle(&msg, sender_key)
    }

    pub fn remove_super(&self, super_address: String, sender_key: Option<&str>) -> Result<GasLog> {
        let msg = HandleMsg::RemoveSuper { super_address };
        self.wrap_handle(&msg, sender_key)
    }
}