use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Coin, CosmosMsg, Empty, QuerierWrapper, StdResult, SubMsg,
    Uint128, WasmMsg,
};
use cw20::{BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};

#[cw_serde]
pub enum Token {
    Denom(String),
    Address(Addr),
}

impl Token {
    pub fn to_key(&self) -> String {
        match self {
            Self::Address(address) => address.to_string(),
            Self::Denom(denom) => denom.clone(),
        }
    }

    pub fn get_denom(&self) -> Option<String> {
        if let Self::Denom(denom) = self {
            Some(denom.clone())
        } else {
            None
        }
    }

    pub fn get_address(&self) -> Option<Addr> {
        if let Self::Address(addr) = self {
            Some(addr.clone())
        } else {
            None
        }
    }

    /// Get the token's balance for the given address.
    pub fn query_balance(
        &self,
        querier: QuerierWrapper<Empty>,
        address: &Addr,
    ) -> StdResult<Uint128> {
        Ok(match self {
            Self::Denom(denom) => querier.query_balance(address.clone(), denom)?.amount,
            Self::Address(cw20_addr) => {
                let BalanceResponse { balance } = querier.query_wasm_smart(
                    cw20_addr.clone(),
                    &Cw20QueryMsg::Balance {
                        address: address.to_string(),
                    },
                )?;
                balance
            },
        })
    }

    /// Send token amount without triggering side-effects
    pub fn transfer(
        &self,
        recipient: &Addr,
        amount: Uint128,
    ) -> StdResult<SubMsg> {
        Ok(match self {
            Self::Denom(denom) => SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.clone().into_string(),
                amount: vec![Coin::new(amount.u128(), denom)],
            })),
            Self::Address(cw20_addr) => SubMsg::new(WasmMsg::Execute {
                contract_addr: cw20_addr.clone().into(),
                msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: recipient.clone().into(),
                    amount,
                })?,
                funds: vec![],
            }),
        })
    }

    pub fn is_in_funds(
        &self,
        funds: &Vec<Coin>,
        amount: Option<Uint128>,
    ) -> bool {
        if let Self::Denom(denom) = self {
            funds
                .iter()
                .find(|c| {
                    c.denom == *denom && (amount.and_then(|n| Some(n == c.amount)).unwrap_or(true))
                })
                .is_some()
        } else {
            false
        }
    }
}

#[cw_serde]
pub struct TokenAmount {
    pub token: Token,
    pub amount: Uint128,
}

#[cw_serde]
pub struct QuoteTokenAmount {
    pub token: Token,
    pub price: Uint128,
    pub amount: Uint128,
    pub boost: Uint128,
    pub y0: Uint128,
}

#[cw_serde]
pub struct TokenInitArgs {
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub supply: Uint128,
    pub cw20: bool,
}
