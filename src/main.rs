use anchor_lang::context::Context;
use anchor_lang::{AnchorDeserialize, AnchorSerialize, zero_copy};
use fixed::types::I80F48;

fn main() {
    println!("Hello, world!");
}

#[zero_copy]
#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct OracleConfig {
    pub conf_filter: I80F48,
    pub max_staleness_slots: i64,
    pub reserved: [u8; 72],
}
