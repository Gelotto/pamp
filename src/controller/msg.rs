use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, Uint128, Uint64};
use std::fmt;

use crate::tokens::{Token, TokenAmount};

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
    ) -> (Uint128, TokenAmount, bool) {
        match self {
            MarketPreset::Juno => {
                let is_mainnet = env.block.chain_id == "juno-1";
                (
                    Uint128::from(1_000_000u128 * ONE_MIL),
                    TokenAmount {
                        amount: Uint128::from(1_000u128 * ONE_MIL),
                        token: Token::Denom(format!("ujuno{}", if is_mainnet { "" } else { "x" })),
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
                    is_mainnet,
                )
            },
        }
    }
}

#[cw_serde]
pub struct ControllerInstantiateMsg {}

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
    Overview {},
    ByOwner(MarketsByOwnerQueryParams),
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
    pub preset: MarketPreset,
    pub tags: Vec<String>,
}

#[cw_serde]
pub struct UpdateMarketMsg {
    pub liquidity: Option<(Uint128, Uint128)>,
    pub pct_supply_remaining: Option<(Uint128, Uint128)>,
    pub volume: Option<(Uint128, Uint128)>,
    pub vote_count: Option<(u32, u32)>,
}

#[cw_serde]
pub struct PublicTokenInitArgs {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
}