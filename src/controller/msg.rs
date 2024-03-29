use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, Uint128, Uint64};
use std::fmt;

use crate::{
    market::msg::BoostParams,
    tokens::{Token, TokenAmount},
};

const ONE_MIL: u128 = 1_000_000;

#[cw_serde]
pub enum MarketPreset {
    Osmosis,
    Juno,
}

impl fmt::Display for MarketPreset {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            MarketPreset::Juno => write!(f, "juno"),
            MarketPreset::Osmosis => write!(f, "osmosis"),
        }
    }
}

impl MarketPreset {
    pub fn parse(
        &self,
        env: &Env,
    ) -> (Uint128, TokenAmount, BoostParams, bool) {
        match self {
            MarketPreset::Juno => {
                let is_mainnet = env.block.chain_id == "juno-1";
                (
                    Uint128::from(1_000_000u128 * ONE_MIL),
                    TokenAmount {
                        amount: Uint128::from(1_000u128 * ONE_MIL),
                        token: Token::Denom(format!("ujuno{}", if is_mainnet { "" } else { "x" })),
                    },
                    BoostParams {
                        burn_pct: Uint128::from(500_000u128),
                        interval_sec: Uint64::from(30u64 * 60u64),
                        min_boost_amount: Uint128::from(1_000_000u128),
                    },
                    is_mainnet,
                )
            },
            MarketPreset::Osmosis => {
                let is_mainnet = env.block.chain_id == "osmosis-1";
                (
                    Uint128::from(1_000_000u128 * ONE_MIL),
                    TokenAmount {
                        amount: Uint128::from(500u128 * ONE_MIL),
                        token: Token::Denom(format!("uosmo{}", if is_mainnet { "" } else { "x" })),
                    },
                    BoostParams {
                        burn_pct: Uint128::from(500_000u128),
                        interval_sec: Uint64::from(30u64 * 60u64),
                        min_boost_amount: Uint128::from(1_000_000u128),
                    },
                    is_mainnet,
                )
            },
        }
    }
}

#[cw_serde]
pub struct ControllerInstantiateMsg {
    pub preset: MarketPreset,
    pub market_code_id: Uint64,
}

#[cw_serde]
pub enum ControllerExecuteMsg {
    CreateMarket(CreateMarketMsg),
    UpdateMarket(UpdateMarketMsg),
}

#[cw_serde]
pub struct MarketsByOwnerQueryParams {
    pub owner: Addr,
    pub cursor: Option<Uint64>,
}

#[cw_serde]
pub enum MarketsQueryMsg {
    Totals {},
    ByOwner(MarketsByOwnerQueryParams),
    InRange(MarketsRangeParams),
}

#[cw_serde]
pub enum MarketIndex {
    CreatedBy,
    CreatedAt,
    UpdatedAt,
    NumSwaps,
    BoostReserves,
    Liquidity,
    QuotePrice,
    SupplyRemainingPercent,
    Volume,
    Tag,
    Symbol,
    Name,
}

#[cw_serde]
pub struct MarketsRangeParams {
    pub index: MarketIndex,
    pub start: Option<String>,
    pub stop: Option<String>,
    pub limit: Option<u8>,
    pub desc: Option<bool>,
    pub cursor: Option<(String, Uint64)>,
}

#[cw_serde]
pub enum ControllerQueryMsg {
    Markets(MarketsQueryMsg),
}

#[cw_serde]
pub struct ControllerMigrateMsg {}

#[cw_serde]
pub struct CreateMarketMsg {
    pub token: PublicTokenInitArgs,
    pub tags: Vec<String>,
}

#[cw_serde]
pub struct UpdateMarketMsg {
    pub price: Option<(Uint128, Uint128)>,
    pub volume: Option<(Uint128, Uint128)>,
    pub liquidity: Option<(Uint128, Uint128)>,
    pub remaining_pct: Option<(Uint128, Uint128)>,
    pub boost: Option<(Uint128, Uint128)>,
    pub n_swaps: Option<(Uint64, Uint64)>,
}

impl UpdateMarketMsg {
    pub fn default() -> Self {
        Self {
            price: None,
            volume: None,
            remaining_pct: None,
            boost: None,
            liquidity: None,
            n_swaps: None,
        }
    }
}

#[cw_serde]
pub struct PublicTokenInitArgs {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
}
