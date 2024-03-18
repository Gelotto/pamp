use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

use crate::{
    bars::Timeframe,
    tokens::{TokenAmount, TokenInitArgs},
};

#[cw_serde]
pub struct MarketInstantiateMsg {
    pub token: TokenInitArgs,
    pub liquidity: TokenAmount,
    pub owner: Option<Addr>,
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
    Reserves {},
    Bars {
        timeframe: Timeframe,
        between: (Option<Timestamp>, Option<Timestamp>),
    },
}
