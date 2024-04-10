use async_trait::async_trait;
use bigdecimal::BigDecimal;

use crate::prelude::*;
use crate::queriers::soulbound::TokenInfo;
use crate::{csv, queriers::liquid::LiquidFinanceCw20, Context};

use super::Exporter;

pub struct LiquidFinance {
    ctx: Arc<Context>,
    csv: csv::Writer<AddressBalance>,
    liquid: LiquidFinanceCw20,
}

impl LiquidFinance {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("liquid-finance").await?;
        let liquid = LiquidFinanceCw20::build(ctx.clone()).await?;

        Ok(Self { ctx, csv, liquid })
    }
}

#[async_trait]
impl Exporter for LiquidFinance {
    #[tracing::instrument(name = "liquid::export", skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting Liquid Finance's sARCH balance");

        let balance = self
            .liquid
            .balance(token.owner.clone())
            .await?
            .with_scale(2);
        let balance_score = balance.to_f64().ok_or(anyhow!(
            "Failed to convert balance amount to f64: {}",
            balance
        ))?;
        let ranking = self
            .ctx
            .ranking
            .ecosystem
            .activities
            .astrovault
            .ranking(balance_score);

        let assets = AddressBalance {
            address: token.owner.clone(),
            balance,
            ranking,
        };

        self.csv.write(assets).await?;

        tracing::info!("Liquid Finance's sARCH balance export finished");

        Ok(())
    }
}

pub struct AddressBalance {
    address: String,
    balance: BigDecimal,
    ranking: f32,
}

impl csv::Item for AddressBalance {
    fn header() -> csv::Header {
        vec!["address", "ranking", "balance"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address.clone(),
            format!("{:.2}", self.ranking),
            self.balance.to_string(),
        ]]
    }
}
