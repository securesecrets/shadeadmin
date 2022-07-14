use ::admin::contract::{execute, instantiate, query};
use anyhow::Result as AnyResult;
use shade_admin::core::{
    serde::{de::DeserializeOwned, Serialize},
    Addr, Coin, ContractInfo, Empty, StdResult,
};
use shade_admin::multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};

#[cfg(test)]
mod test;

/// Wrapper integrating admin auth with multi-test.
pub struct AdminAuth {
    pub info: ContractInfo,
}

/// Trait for making integration with multi-test easier.
pub trait MultiTestable {
    fn get_info(&self) -> &ContractInfo;
    fn contract() -> Box<dyn Contract<Empty>>;
    fn new(info: ContractInfo) -> Self;
    fn init<T: Serialize>(
        router: &mut App,
        sender: Addr,
        label: &str,
        send_funds: &[Coin],
        msg: &T,
    ) -> ContractInfo {
        let stored_code = router.store_code(AdminAuth::contract());
        router
            .instantiate_contract(stored_code, sender, &msg, send_funds, label, None)
            .unwrap()
    }
    fn query<T: DeserializeOwned>(&self, router: &App, msg: &impl Serialize) -> StdResult<T> {
        let info = self.get_info();
        router
            .wrap()
            .query_wasm_smart(info.code_hash.clone(), info.address.clone(), &msg)
    }
    fn execute<T: Serialize + std::fmt::Debug>(
        &self,
        router: &mut App,
        sender: Addr,
        msg: &T,
        send_funds: &[Coin],
    ) -> AnyResult<AppResponse> {
        router.execute_contract(sender, (*self.get_info()).clone(), msg, send_funds)
    }
}

impl MultiTestable for AdminAuth {
    fn get_info(&self) -> &ContractInfo {
        &self.info
    }

    fn contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new_with_empty(execute, instantiate, query);
        Box::new(contract)
    }

    fn new(info: ContractInfo) -> Self {
        AdminAuth { info }
    }
}
