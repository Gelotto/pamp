use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128, Uint64};

pub const MIN_BOOST_SECONDS: u64 = 5 * 60;
pub const MAX_BOOST_SECONDS: u64 = 60 * 60 * 2 * 24;

pub const DEFAULT_BOOST_BURN_PCT: u128 = 500_000;
pub const DEFAULT_BOOST_SECONDS: u64 = 30 * 60;
pub const DEFAULT_MIN_BOOST_AMOUNT: u128 = 1000;

use crate::{
    bars::Timeframe,
    boost::Boost,
    tokens::{TokenAmount, TokenInitArgs},
};

#[cw_serde]
pub struct MarketInstantiateMsg {
    pub token: TokenInitArgs,
    pub liquidity: TokenAmount,
    pub owner: Option<Addr>,
    pub boosts: Option<BoostParams>,
}

#[cw_serde]
pub struct BoostParams {
    pub interval_sec: Uint64,
    pub min_boost_amount: Uint128,
    pub burn_pct: Uint128,
}

impl BoostParams {
    pub fn get_interval_key(
        &self,
        time: Timestamp,
    ) -> Uint64 {
        time.minus_seconds(time.seconds() % self.interval_sec.u64())
            .seconds()
            .into()
    }

    pub fn sanitize(unclean_params: &Option<Self>) -> Self {
        unclean_params
            .to_owned()
            .and_then(|mut params| {
                params.burn_pct = Uint128::from(params.burn_pct.u128().clamp(0u128, 1_000_000u128));
                params.min_boost_amount = Uint128::from(
                    params
                        .min_boost_amount
                        .u128()
                        .clamp(DEFAULT_MIN_BOOST_AMOUNT, u128::MAX),
                );
                params.interval_sec = Uint64::from(
                    params
                        .interval_sec
                        .u64()
                        .clamp(MIN_BOOST_SECONDS, MAX_BOOST_SECONDS),
                );
                Some(params)
            })
            .unwrap_or_else(|| Self {
                burn_pct: Uint128::from(DEFAULT_BOOST_BURN_PCT),
                interval_sec: Uint64::from(DEFAULT_BOOST_SECONDS),
                min_boost_amount: Uint128::from(DEFAULT_MIN_BOOST_AMOUNT),
            })
    }

    pub fn init_boost(
        &self,
        time: Timestamp,
    ) -> Boost {
        Boost {
            interval_key: self.get_interval_key(time),
            reserves: Uint128::zero(),
            swapped: Uint128::zero(),
            increment: Uint128::zero(),
            updated_at: None,
            count: 0,
        }
    }
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
