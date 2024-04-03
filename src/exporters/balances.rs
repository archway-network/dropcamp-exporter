use std::collections::HashMap;

use crate::{csv, Context};
use anyhow::Result;
use async_trait::async_trait;

use super::Exporter;

pub struct Balances {
    ctx: Context,
    csv: csv::Writer<AddressBalances>,
}

impl Balances {
    pub async fn create(ctx: Context) -> Result<Self> {
        let csv = ctx.csv_writer(Self::NAME).await?;
        Ok(Self { ctx, csv })
    }
}

#[async_trait]
impl Exporter for Balances {
    const NAME: &'static str = "balances";

    #[tracing::instrument(skip(self))]
    async fn export(&self, address: &str) -> Result<()> {
        tracing::debug!("exporting all balances");

        let response = self.ctx.cosmos.bank.balances(address.to_string()).await?;
        let balances: HashMap<String, String> = response
            .balances
            .into_iter()
            .map(|coin| (coin.denom, coin.amount))
            .collect();

        let assets = AddressBalances {
            address: address.to_string(),
            balances,
        };

        self.csv.write(assets).await?;

        Ok(())
    }
}

pub struct AddressBalances {
    address: String,
    balances: HashMap<String, String>,
}

impl csv::Item for AddressBalances {
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
