use colored::*;
use secretcli::cli_types::NetContract;
use secretcli::secretcli::{account_address, query_contract, test_contract_handle, test_inst_init};
use serde_json::Result;
use shade_admin::admin::{
    InitMsg, HandleMsg, QueryMsg, SuperAdminResponse,
    ContractsResponse, AuthorizedUsersResponse, ValidateAuthorityResponse
};
use cosmwasm_std::HumanAddr;
use crate::constants::{*, testnet::*};
use crate::contract_helpers::{TestableContract, Contract, admin::AdminAuthContract};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test1() {

    let ensemble = Rc::new(RefCell::new(ContractEnsemble::new(50)));
    let overseer = ensemble.borrow_mut().register(Box::new(AdminAuthHarness));
    //let admin_auth = Contract::new(ADMIN.to_string(), ADMIN_HASH.to_string());

    println!("Deploying Admin Auth Contract.");
    let admin_auth = AdminAuthContract::new(
        &InitMsg { },
        Some(HOOMP_KEY),
        None,
    ).unwrap();
    assert!(true == false);

    println!("Adding CONTRACT1.");
    let txn = admin_auth.add_contract(
        "CONTRACT1".to_string(),
        Some(HOOMP_KEY),
    ).unwrap();
    //println!("Completed tx: {}", txn);

    let super_list = admin_auth.query_super_admins().unwrap().super_admins;
    println!("Super-admins: {:?}", super_list);

    /*
    println!("Deploying stkd-SCRT oracle.");
    let stkd_scrt_oracle = ShadeStakingDerivativeOracleContract::new(
        &shd_stkd::InitMsg {
            owner: HumanAddr(user_a.clone()),
            supported_symbol: "stkd-SCRT".to_string(),
            underlying_symbol: "SCRT".to_string(),
            staking_derivative,
            router: router.as_contract(),
        },
        Some(HOOMP_KEY),
        Some("stkd_scrt_oracle"),
    )?;

    println!("Registering stkd-SCRT oracle to router.");
    router.update_registry(
        RegistryOperation::Add {
            oracle: stkd_scrt_oracle.as_contract(),
            key: "stkd-SCRT".to_string(),
        },
        Some(HOOMP_KEY),
    )?;

    println!("Deploying stkd-SCRT / SCRT Siennaswap LP oracle.");
    let stkd_scrt_scrt_lp_oracle = SiennaswapSpotLpOracleContract::new(
        &SiennaSwapLpOracle::InitMsg {
            owner: HumanAddr(user_a),
            symbol_0: "stkd-SCRT".to_string(),
            symbol_1: "SCRT".to_string(),
            router: router.as_contract(),
            factory: sienna_stkd_scrt_scrt_lp,
            supported_symbol: "stkd-SCRT/SCRT SiennaSwap LP".to_string(),
        },
        Some(HOOMP_KEY),
        Some("stkd_scrt_scrt_lp_oracle"),
    )?;

    println!("Registering stkd-SCRT/SCRT oracle to router.");
    router.update_registry(
        RegistryOperation::Replace {
            oracle: stkd_scrt_scrt_lp_oracle.as_contract(),
            key: "stkd-SCRT/SCRT SiennaSwap LP".to_string(),
        },
        Some(HOOMP_KEY),
    )?;

    */

}
