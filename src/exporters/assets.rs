use std::collections::HashMap;

use crate::{csv, Context};
use anyhow::Result;
use async_trait::async_trait;

use super::Exporter;

pub struct AssetsExporter {
    ctx: Context,
    csv: csv::Writer<AddressAssets>,
}

impl AssetsExporter {
    pub async fn create(ctx: Context) -> Result<Self> {
        let csv = ctx.csv_writer(Self::NAME).await?;
        Ok(Self { ctx, csv })
    }
}

#[async_trait]
impl Exporter for AssetsExporter {
    const NAME: &'static str = "assets";

    #[tracing::instrument(skip(self))]
    async fn export(&self, address: &str) -> Result<()> {
        tracing::debug!("exporting bridged assets");

        let response = self.ctx.cosmos.bank.balances(address.to_string()).await?;
        let balances: HashMap<String, String> = response
            .balances
            .into_iter()
            .map(|coin| (coin.denom, coin.amount))
            .collect();

        let assets = AddressAssets {
            address: address.to_string(),
            balances,
        };

        self.csv.write(assets).await?;

        Ok(())
    }
}

pub struct AddressAssets {
    address: String,
    balances: HashMap<String, String>,
}

impl csv::Item for AddressAssets {
    fn header() -> csv::Header {
        vec!["address", "denom", "amount"]
    }

    fn rows(self) -> Vec<csv::Row> {
        let mut rows = vec![];
        for (denom, amount) in self.balances {
            let row = vec![self.address.clone(), denom, amount];
            rows.push(row);
        }
        rows
    }
}
