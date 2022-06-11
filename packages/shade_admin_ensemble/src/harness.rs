#[cfg(feature = "admin")]
pub mod admin {
    use ensemble_harness::helpers;
    use admin;

    pub struct SiennaSwapLpOracle;
    helpers::implement_harness!(SiennaSwapLpOracle, siennaswap_lp_spot_oracle);
}