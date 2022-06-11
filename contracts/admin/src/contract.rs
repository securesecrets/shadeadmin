use shade_admin::admin::{InitMsg, HandleMsg, QueryMsg, SuperAdminResponse, ContractsResponse, AuthorizedUsersResponse, ValidateAuthorityResponse};
use shade_admin::storage::{Map, Item};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let super_admin: Vec<String> = vec![env.message.sender.to_string()];
    SUPER.save(&mut deps.storage, &super_admin)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddContract { contract_hash } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let contract = CONTRACT.may_load(&deps.storage, contract_hash.clone())?;
            match contract {
                Some(_contract) => Err(StdError::generic_err("Contract already exists.")),
                None => {
                    CONTRACT.save(&mut deps.storage, contract_hash.clone(), &Vec::new())?;
                    let mut keys = KEYS.load(&deps.storage)?;
                    keys.push(contract_hash);
                    KEYS.save(&mut deps.storage, &keys)?;
                    Ok(HandleResponse::default())
                },
            }
        },
        HandleMsg::RemoveContract { contract_hash } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let contract = CONTRACT.may_load(&deps.storage, contract_hash.clone())?;
            match contract {
                Some(_contract) => {
                    CONTRACT.remove(&mut deps.storage, contract_hash.clone());
                    let mut keys = KEYS.load(&deps.storage)?;
                    keys.retain(|k| k != &contract_hash);
                    KEYS.save(&mut deps.storage, &keys)?;
                    Ok(HandleResponse::default())
                },
                None => Err(StdError::generic_err("Contract does not exist.")),
            }
        },
        HandleMsg::AddAuthorization { contract_hash, admin_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let user_list = CONTRACT.may_load(&deps.storage, contract_hash.clone())?;
            match user_list {
                Some(mut user_list) => {
                    if user_list.iter().any(|x| x == &admin_address) {
                        Err(StdError::generic_err("Admin address already exists."))
                    } else {
                        user_list.push(admin_address);
                        CONTRACT.save(&mut deps.storage, contract_hash, &user_list)?;
                        Ok(HandleResponse::default())
                    }
                },
                None => Err(StdError::generic_err("Contract does not exist.")),
            }
        },
        HandleMsg::RemoveAuthorization { contract_hash, admin_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let user_list = CONTRACT.may_load(&deps.storage, contract_hash.clone())?;
            match user_list {
                Some(mut user_list) => {
                    if user_list.iter().any(|x| x == &admin_address) {
                        user_list.retain(|k| k != &admin_address);
                        CONTRACT.save(&mut deps.storage, contract_hash, &user_list)?;
                        Ok(HandleResponse::default())
                    } else {
                        Err(StdError::generic_err("Admin address was never authorized."))
                    }
                },
                None => Err(StdError::generic_err("Contract does not exist.")),
            }
        },
        HandleMsg::AddSuper { super_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let mut super_list = SUPER.load(&deps.storage)?;
            if super_list.iter().any(|x| x == &super_address) {
                Err(StdError::generic_err("Super-admin address already exists."))
            } else {
                super_list.push(super_address);
                SUPER.save(&mut deps.storage, &super_list)?;
                Ok(HandleResponse::default())
            }
        },
        HandleMsg::RemoveSuper { super_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let mut super_list = SUPER.load(&deps.storage)?;
            if super_list.iter().any(|x| x == &super_address) {
                super_list.retain(|k| k != &super_address);
                SUPER.save(&mut deps.storage, &super_list)?;
                Ok(HandleResponse::default())
            } else {
                Err(StdError::generic_err("Super-admin address was never authorized."))
            }
        }
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSuperAdmins { } => {
            let super_admins = SUPER.load(&deps.storage)?;
            to_binary(& SuperAdminResponse{
                super_admins: super_admins
            })
        },
        QueryMsg::GetContracts { } => {
            let mut contracts_result = vec![];
            let keys = KEYS.load(&deps.storage)?;
            for key in keys.iter() {
                let admin_list = CONTRACT.load(&deps.storage, key.clone())?;
                contracts_result.push((key.clone(), admin_list));
            }
            to_binary(& ContractsResponse{
                contracts: contracts_result
            })
        },
        QueryMsg::GetAuthorizedUsers { contract_hash } => {
            let authorized_users = CONTRACT.load(&deps.storage, contract_hash)?;
            to_binary(& AuthorizedUsersResponse{
                authorized_users: authorized_users
            })
        },
        QueryMsg::ValidateAuthority { contract_hash, admin_address } => {
            let error_msg = is_authorized(&deps.storage, contract_hash, admin_address);
            to_binary(& ValidateAuthorityResponse{
                error_msg: error_msg
            })
        },
    }
}

fn is_super(storage: &impl Storage, address: &String) -> StdResult<()> {
    let super_admins = SUPER.load(storage)?;
    if super_admins.iter().any(|k| k == address) {
        Ok(())
    } else {
        Err(StdError::unauthorized())
    }
}

fn is_authorized(storage: &impl Storage, contract_hash: String, admin_address: String) -> Option<String> {
	let user_list = CONTRACT.load(storage, contract_hash).ok()?;
	let super_admins = SUPER.load(storage).ok()?;
	if super_admins.iter().any(|k| k == &admin_address) || user_list.iter().any(|k| k == &admin_address) {
		None
    } else {
        Some("Not authorized.".to_string())
    }
}

const CONTRACT: Map<String, Vec<String>> = Map::new("contract");
const KEYS: Item<Vec<String>> = Item::new("keys");
const SUPER: Item<Vec<String>> = Item::new("super");