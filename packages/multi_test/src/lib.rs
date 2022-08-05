#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test;

#[cfg(not(target_arch = "wasm32"))]
pub mod multi {
    multi_derive::implement_multi!(AdminAuth, admin);
    pub use shade_admin::admin::*;

    use shade_admin::InstantiateCallback;
    use shade_protocol::multi_test::App;

    pub mod helpers {
        use super::*;
        /// Initializes an admin auth contract in multitest with superadmin as the superadmin.
        pub fn init_admin_auth(app: &mut App, superadmin: &Addr) -> ContractInfo {
            InstantiateMsg {
                super_admin: Some(superadmin.clone().to_string()),
            }
            .test_init(
                AdminAuth::default(),
                app,
                superadmin.clone(),
                "admin_auth",
                &[],
            )
            .unwrap()
        }
    }
}
