use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};

#[cw_serde]
pub struct Bar {
    /// open at
    pub t: Timestamp,
    /// open
    pub o: Uint128,
    /// close
    pub c: Uint128,
    /// high
    pub h: Uint128,
    /// low
    pub l: Uint128,
    /// volume
    pub v: Uint128,
    /// num trades
    pub n: u32,
}

#[cw_serde]
pub enum Timeframe {
    M5,
    H1,
}

impl Timeframe {
    pub fn to_seconds(&self) -> u64 {
        match self {
            Self::M5 => 5u64 * 60u64,
            Self::H1 => 60u64 * 60u64,
        }
    }
}
