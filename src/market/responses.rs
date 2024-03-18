use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty, Uint128, Uint64};

use crate::{
    bars::Bar,
    tokens::{QuoteTokenAmount, TokenAmount},
};

#[cw_serde]
pub struct ConfigResponse(pub Empty);

#[cw_serde]
pub struct BarsResponse(pub Vec<Bar>);

#[cw_serde]
pub struct MarketsResponse {
    pub n_markets: Uint64,
}

#[cw_serde]
pub struct MarketAddressesResponse {
    pub markets: Vec<Addr>,
    pub cursor: Option<Uint64>,
}

#[cw_serde]
pub struct ReservesResponse {
    pub base: TokenAmount,
    pub quote: QuoteTokenAmount,
    pub k: Uint128,
}
