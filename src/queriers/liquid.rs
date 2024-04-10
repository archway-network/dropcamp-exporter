use std::str::FromStr;

use bigdecimal::{num_bigint::BigUint, BigDecimal};
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum QueryMsg {
    Balance { address: String },
    TokenInfo {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BalanceResponse {
    pub balance: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
}

pub struct LiquidFinanceCw20 {
    ctx: Arc<Context>,
    token_info: TokenInfoResponse,
}

impl LiquidFinanceCw20 {
    pub async fn build(ctx: Arc<Context>) -> Result<Self> {
        let token_info = Self::token_info(&ctx).await?;
        tracing::debug!(
            address = ctx.liquid_finance_address,
            symbol = token_info.symbol,
            decimals = token_info.decimals,
            "building cw20 token"
        );

        Ok(Self { ctx, token_info })
    }

    #[tracing::instrument(skip(self))]
    pub async fn balance(&self, address: String) -> Result<BigDecimal> {
        tracing::debug!(%self.ctx.liquid_finance_address, "fetching cw20 token balance");

        let query = QueryMsg::Balance { address };
        let response: BalanceResponse = self
            .ctx
            .query_contract(self.ctx.liquid_finance_address.clone(), &query)
            .await?;
        tracing::debug!(balance = response.balance, "cw20 token balance");

        let balance_digits = BigUint::from_str(response.balance.as_str())?;
        let balance = BigDecimal::new(balance_digits.into(), self.token_info.decimals as i64);

        Ok(balance)
    }

    async fn token_info(ctx: &Arc<Context>) -> Result<TokenInfoResponse> {
        let query = QueryMsg::TokenInfo {};
        ctx.query_contract(ctx.liquid_finance_address.clone(), &query)
            .await
    }
}
