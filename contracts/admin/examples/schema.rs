use shade_admin::core::cosmwasm_schema::write_api;
use shade_admin::core::cosmwasm_schema as cosmwasm_schema;
use shade_admin::admin::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}