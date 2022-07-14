#[cfg(test)]
mod test;

pub mod admin {
    pub struct AdminAuth {
        pub info: ContractInfo,
    }

    use admin;
    multi_derive::implement_multi!(AdminAuth, admin);
}
