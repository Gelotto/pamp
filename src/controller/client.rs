use cosmwasm_std::{to_json_binary, Addr, StdResult, WasmMsg};

use super::msg::{ControllerExecuteMsg, UpdateMarketMsg};

pub fn update_market_indices_msg(
    contoller_addr: Addr,
    msg: UpdateMarketMsg,
) -> StdResult<WasmMsg> {
    Ok(WasmMsg::Execute {
        contract_addr: contoller_addr.into_string(),
        msg: to_json_binary(&ControllerExecuteMsg::UpdateMarket(msg))?,
        funds: vec![],
    })
}
