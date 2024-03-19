use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Timestamp, Uint128, Uint64};

#[cw_serde]
pub struct Boost {
    pub interval_key: Uint64,
    pub reserves: Uint128,
    pub swapped: Uint128,
    pub updated_at: Option<Timestamp>,
    pub increment: Uint128,
    pub count: u32,
}

impl Boost {
    /// Calculate the number of boost time intervals that fit within the time
    /// window between the boost's creation and the present `now` time. Then
    /// calculate the cumulative boost amount over that number of intervals. The
    /// upper limit of the boost amount is clamped to the remaining boost
    /// reserves amount (boost.reserves - boost.swapped).
    ///
    /// In pseudocode:
    ///     delta_seconds = (now - prev_time).seconds()
    ///     n_intervals = delta_seconds / interval_seconds
    ///     net_boost_amount = min(boost.incr * n_intervals, boost.reserves)
    pub fn calculate_vesting_over(
        &self,
        now: Timestamp,
        interval_seconds: Uint64,
    ) -> StdResult<Uint128> {
        // Trim subsecond resolution on the timestamps used by boosting
        let now = Timestamp::from_seconds(now.seconds());

        let total_seconds = Uint128::from(if let Some(prev_time) = self.updated_at {
            now.minus_seconds(prev_time.seconds()).seconds() as u128
        } else {
            0u128
        });

        let n_intervals = total_seconds.checked_div(Uint128::from(interval_seconds.u64()))?;

        let net_boost_amount = n_intervals
            .checked_mul(self.increment)
            .unwrap_or(Uint128::zero())
            .min(self.reserves.checked_sub(self.swapped)?);

        Ok(net_boost_amount)
    }
}
