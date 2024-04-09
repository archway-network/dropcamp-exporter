use async_trait::async_trait;

use crate::prelude::*;
use crate::queriers::soulbound::TokenInfo;
use crate::{csv, queriers::liquid::LiquidFinanceCw20, Context};

use super::Exporter;

pub struct LiquidFinance {
    csv: csv::Writer<AddressBalance>,
    liquid: LiquidFinanceCw20,
}

impl LiquidFinance {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("liquid-finance").await?;
        let liquid = LiquidFinanceCw20::new(ctx);

        Ok(Self { csv, liquid })
    }
}

#[async_trait]
impl Exporter for LiquidFinance {
    #[tracing::instrument(skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting Liquid Finance's sARCH balance");

        let balance = self.liquid.get_balance(token.owner.clone()).await?;
        let assets = AddressBalance {
            address: token.owner.clone(),
            balance,
        };

        self.csv.write(assets).await?;

        tracing::info!("Liquid Finance's sARCH balance export finished");

        Ok(())
    }
}

pub struct AddressBalance {
    address: String,
    balance: String,
}

impl csv::Item for AddressBalance {
    fn header() -> csv::Header {
        vec!["address", "balance"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![self.address.clone(), self.balance]]
    }
}
