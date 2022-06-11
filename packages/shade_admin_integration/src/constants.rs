// Math
pub const DECIMAL_FACTOR: u128 = 10u128.pow(6);

// Smart contracts
pub const STORE_GAS: &str = "10000000";
pub const GAS: &str = "800000";
pub const VIEW_KEY: &str = "password";

pub const ADMIN_AUTH_FILE: &str = "../../compiled/admin.wasm.gz";

// Default executer & admin address for testing
pub const USER_A_KEY: &str = "a";
pub const USER_B_KEY: &str = "b";
pub const USER_C_KEY: &str = "c";
pub const USER_D_KEY: &str = "d";
pub const HOOMP_KEY: &str = "hoomp";
pub const BACKEND: &str = "test";

pub mod testnet {
    pub const ADMIN: &str = "secret1ulxxh6erkmk4p6cjehz58cqspw3qjuedrsxp8f";
    pub const ADMIN_HASH: &str = "DC6FF596E1CD83B84A6FFBD857576D7693D89A826471D58E16349015E412A3D3";
}
