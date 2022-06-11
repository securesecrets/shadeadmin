use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg { }

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum HandleMsg {
	AddContract{
		contract_hash: String
	},
	RemoveContract{
		contract_hash: String
	},
	AddAuthorization {
		contract_hash: String,
		admin_address: String
	},
	RemoveAuthorization { 
		contract_hash: String,
		admin_address: String
	},
	AddSuper {
		super_address: String
	},
	RemoveSuper {
		super_address: String
	}
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum QueryMsg {
	GetSuperAdmins { },
	GetContracts { },
	GetAuthorizedUsers { contract_hash: String },
	ValidateAuthority {
		contract_hash: String,
		admin_address: String
	},
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SuperAdminResponse {
	pub super_admins: Vec<String>
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractsResponse {
	pub contracts: Vec<(String, Vec<String>)>
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AuthorizedUsersResponse {
	pub authorized_users: Vec<String>
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ValidateAuthorityResponse {
	pub error_msg: Option<String>
}