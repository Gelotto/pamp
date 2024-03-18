use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Timestamp, Uint128, Uint64};

#[cw_serde]
pub struct Boost {
    pub reserves: Uint128,
    pub t: Option<Timestamp>,
    pub incr: Uint128,
    pub count: u32,
}

impl Boost {
    pub fn get_aggregate_boost_amount(
        &self,
        now: Timestamp,
        interval_sec: Uint64,
    ) -> StdResult<Uint128> {
        let now = Timestamp::from_seconds(now.seconds());

        let seconds_ellapsed = Uint128::from(if let Some(prev_time) = self.t {
            now.minus_seconds(prev_time.seconds()).seconds() as u128
        } else {
            0u128
        });

        let n_ellapsed_intervals =
            seconds_ellapsed.checked_div(Uint128::from(interval_sec.u64()))?;

        Ok(n_ellapsed_intervals
            .checked_mul(self.incr)
            .unwrap_or(Uint128::zero())
            .min(self.reserves))
    }
}
