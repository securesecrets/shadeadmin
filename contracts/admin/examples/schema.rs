use shade_admin::admin::{ExecuteMsg, InstantiateMsg, QueryMsg};
use shade_admin::core::cosmwasm_schema;
use shade_admin::core::cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
