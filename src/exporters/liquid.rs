use crate::{csv, queriers::liquid::LiquidFinanceCw20, Context};
use anyhow::Result;
use async_trait::async_trait;

use super::Exporter;

pub struct LiquidFinance {
    csv: csv::Writer<AddressBalance>,
    liquid: LiquidFinanceCw20,
}

impl LiquidFinance {
    pub async fn create(ctx: Context) -> Result<Self> {
        let csv = ctx.csv_writer("liquid-finance").await?;
        let liquid = LiquidFinanceCw20::new(ctx);

        Ok(Self { csv, liquid })
    }
}

#[async_trait]
impl Exporter for LiquidFinance {
    #[tracing::instrument(skip(self))]
    async fn export(&self, address: &str) -> Result<()> {
        tracing::debug!("exporting ArchID domains");

        let balance = self.liquid.get_balance(address.to_string()).await?;
        let assets = AddressBalance {
            address: address.to_string(),
            balance,
        };

        self.csv.write(assets).await?;

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