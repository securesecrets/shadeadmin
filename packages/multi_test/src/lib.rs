#[cfg(test)]
mod test;

multi_derive::implement_multi!(AdminAuth, admin);
pub use shade_admin::admin::*;