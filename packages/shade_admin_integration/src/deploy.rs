use colored::*;
use secretcli::cli_types::NetContract;
use secretcli::secretcli::{account_address, query_contract, test_contract_handle, test_inst_init};
use serde_json::Result;
use shade_admin::admin::{
    InitMsg, HandleMsg, QueryMsg, SuperAdminResponse,
    ContractsResponse, AuthorizedUsersResponse, ValidateAuthorityResponse
};
use cosmwasm_std::HumanAddr;
use shade_admin_integration::constants::testnet::*;
use shade_admin_integration::constants::*;
use shade_admin_integration::contract_helpers::{TestableContract, Contract};
use shade_admin_integration::contract_helpers::admin::AdminAuthContract;

#[test]
fn main() -> Result<()> {
    let user_a = account_address(HOOMP_KEY).unwrap_or_default();

    println!("Account A: {}", user_a.blue());

    deploy(user_a)?;
    Ok(())
}

fn deploy_test(user_a: String) -> Result<()> {
    // let admin_auth = Contract {
    //     address: HumanAddr::from(ADMIN.to_string()),
    //     code_hash: ADMIN_HASH.to_string(),
    // };

    println!("Deploying Admin Auth Contract.");
    let admin_auth = AdminAuthContract::new(
        &InitMsg { },
        Some(HOOMP_KEY),
        Some("admin_auth"),
    )?;

    Ok(())
}

fn deploy(user_a: String) -> Result<()> {

    //let admin_auth = Contract::new(ADMIN.to_string(), ADMIN_HASH.to_string());

    println!("Deploying Admin Auth Contract.");
    let admin_auth = AdminAuthContract::new(
        &InitMsg { },
        Some(HOOMP_KEY),
        Some("admin_auth"),
    )?;

    println!("Adding CONTRACT1.");
    let txn = admin_auth.add_contract(
        "CONTRACT1".to_string(),
        Some(HOOMP_KEY),
    )?.txhash;
    println!("Completed tx: {}", txn);

    let super_list = admin_auth.query_super_admins()?.super_admins;
    println!("Super-admins: {:?}", super_list);

    println!("Adding CONTRACT1.");
    admin_auth.add_contract(
        "CONTRACT1".to_string(),
        Some(USER_A_KEY),
    )?;
    assert!(true == false);

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

    Ok(())
}
