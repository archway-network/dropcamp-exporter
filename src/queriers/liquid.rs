use serde::{de::DeserializeOwned, Deserialize, Serialize};

use anyhow::*;

use crate::context::Context;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum QueryMsg {
    Balance { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BalanceResponse {
    pub balance: String,
}

pub struct LiquidFinanceCw20 {
    ctx: Context,
}

impl LiquidFinanceCw20 {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_balance(&self, address: String) -> Result<String> {
        tracing::debug!(%self.ctx.liquid_finance_address, "fethcing cw20 token balance");

        let query = QueryMsg::Balance { address };
        let BalanceResponse { balance } = self.query_contract(&query).await?;
        tracing::debug!(%balance, "cw20 token balance");

        Ok(balance)
    }

    async fn query_contract<T, R>(&self, data: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        self.ctx
            .cosmos
            .cosmwasm
            .smart_contract_state(self.ctx.liquid_finance_address.clone(), data)
            .await
    }
}
