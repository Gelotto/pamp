use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128, Uint64};

use crate::{
    bars::Timeframe,
    tokens::{TokenAmount, TokenInitArgs},
};

#[cw_serde]
pub struct MarketInstantiateMsg {
    pub token: TokenInitArgs,
    pub liquidity: TokenAmount,
    pub owner: Option<Addr>,
    pub boosts: Option<BoostingParams>,
}

#[cw_serde]
pub struct BoostingParams {
    pub interval_sec: Uint64,
    pub min_boost_amount: Uint128,
    pub burn_pct: Uint128,
}

#[cw_serde]
pub enum MarketExecuteMsg {
    Buy { quote_amount: Uint128 },
    Sell { base_amount: Uint128 },
}

#[cw_serde]
pub enum MarketMigrateMsg {}

#[cw_serde]
pub enum MarketQueryMsg {
    Pool {},
    Bars {
        timeframe: Timeframe,
        between: (Option<Timestamp>, Option<Timestamp>),
        cursor: Option<Uint64>,
    },
}
