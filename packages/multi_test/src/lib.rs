#[cfg(test)]
mod test;

pub mod admin {
    pub struct AdminAuth {
        pub info: ContractInfo,
    }

    use admin;
    shade_multi_test::multi_macro::implement_multi!(AdminAuth, admin);
}
