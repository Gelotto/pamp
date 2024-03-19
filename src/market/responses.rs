use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty, Uint128, Uint256, Uint64};

use crate::{
    bars::Bar,
    boost::Boost,
    tokens::{QuoteTokenAmount, TokenAmount},
};

#[cw_serde]
pub struct ConfigResponse(pub Empty);

#[cw_serde]
pub struct BarsResponse(pub Vec<Bar>);

#[cw_serde]
pub struct MarketTotalsResponse {
    pub n_markets: Uint64,
    pub n_swaps: Uint128,
    pub liquidity: Uint256,
    pub volume: Uint256,
    pub boost: Uint256,
}

#[cw_serde]
pub struct MarketAddressesResponse {
    pub markets: Vec<Addr>,
    pub cursor: Option<Uint64>,
}

#[cw_serde]
pub struct MarketsRangeResponse {
    pub markets: Vec<Addr>,
    pub cursor: Option<(String, Uint64)>,
}

#[cw_serde]
pub struct PoolResponse {
    pub base: TokenAmount,
    pub quote: QuoteTokenAmount,
    pub boost: Boost,
    pub k: Uint128,
}
