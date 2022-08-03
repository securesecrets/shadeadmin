#![feature(is_some_with)]
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test;

#[cfg(not(target_arch = "wasm32"))]
pub mod multi {
    multi_derive::implement_multi!(AdminAuth, admin);
    pub use shade_admin::admin::*;
}
