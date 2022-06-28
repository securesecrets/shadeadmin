use shade_admin::admin::{InitMsg, HandleMsg, QueryMsg, SuperAdminResponse, ContractsResponse, AuthorizedUsersResponse, ValidateAdminPermissionResponse};
use shade_admin::storage::{Map, Item};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

const CONTRACT: Map<String, Vec<String>> = Map::new("contract");
const KEYS: Item<Vec<String>> = Item::new("keys");
const SUPER: Item<Vec<String>> = Item::new("super");

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let super_admin: Vec<String> = vec![env.message.sender.to_string()];
    SUPER.save(&mut deps.storage, &super_admin)?;
    KEYS.save(&mut deps.storage, &Vec::new())?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddContract { contract_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let contract = CONTRACT.may_load(&deps.storage, contract_address.clone())?;
            match contract {
                Some(_contract) => Err(StdError::generic_err("Contract already exists.")),
                None => {
                    CONTRACT.save(&mut deps.storage, contract_address.clone(), &Vec::new())?;
                    let mut keys = KEYS.load(&deps.storage)?;
                    keys.push(contract_address);
                    KEYS.save(&mut deps.storage, &keys)?;
                    Ok(HandleResponse::default())
                },
            }
        },
        HandleMsg::RemoveContract { contract_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            contract_exists(CONTRACT.may_load(&deps.storage, contract_address.clone())?)?;
            CONTRACT.remove(&mut deps.storage, contract_address.clone());
            let mut keys = KEYS.load(&deps.storage)?;
            keys.retain(|k| k != &contract_address);
            KEYS.save(&mut deps.storage, &keys)?;
            Ok(HandleResponse::default())
        },
        HandleMsg::AddAuthorization { contract_address, admin_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let mut user_list = contract_exists(CONTRACT.may_load(&deps.storage, contract_address.clone())?)?;
            if user_list.iter().any(|x| x == &admin_address) {
                Err(StdError::generic_err("Admin address already exists."))
            } else {
                user_list.push(admin_address);
                CONTRACT.save(&mut deps.storage, contract_address, &user_list)?;
                Ok(HandleResponse::default())
            }
        },
        HandleMsg::RemoveAuthorization { contract_address, admin_address } => {
            is_super(&deps.storage, &env.message.sender.to_string())?;
            let mut user_list = contract_exists(CONTRACT.may_load(&deps.storage, contract_address.clone())?)?;
            if user_list.iter().any(|x| x == &admin_address) {
                user_list.retain(|k| k != &admin_address);
                CONTRACT.save(&mut deps.storage, contract_address, &user_list)?;
                Ok(HandleResponse::default())
            } else {
                Err(StdError::generic_err("Admin address was never authorized."))
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
        QueryMsg::GetAuthorizedUsers { contract_address } => {
            let authorized_users = CONTRACT.load(&deps.storage, contract_address)?;
            to_binary(& AuthorizedUsersResponse{
                authorized_users: authorized_users
            })
        },
        QueryMsg::ValidateAdminPermission { contract_address, admin_address } => {
            let error_msg = is_authorized(&deps.storage, contract_address, admin_address)?;
            to_binary(& ValidateAdminPermissionResponse{
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

fn is_authorized(storage: &impl Storage, contract_address: String, admin_address: String) -> StdResult<Option<String>> {
	let super_admins = SUPER.load(storage)?;
	if super_admins.iter().any(|k| k == &admin_address) {
		Ok(None)
    } else {
        let user_list = CONTRACT.may_load(storage, contract_address)?;
        if let Some(user_list) = user_list {
            if user_list.iter().any(|k| k == &admin_address) {
                Ok(None)
            } else {
                Ok(Some("Not authorized.".to_string()))
            }
        } else {
            Ok(Some("Contract does not exist within the Admin Auth.".to_string()))
        }
    }
}

fn contract_exists(user_list: Option<Vec<String>>) -> StdResult<Vec<String>> {
    if let Some(user_list) = user_list {
        Ok(user_list)
    } else {
        Err(StdError::generic_err("Contract does not exist."))
    }
}