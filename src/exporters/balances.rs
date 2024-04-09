use std::collections::HashMap;

use async_trait::async_trait;

use crate::prelude::*;
use crate::queriers::soulbound::TokenInfo;
use crate::{csv, Context};

use super::Exporter;

pub struct Balances {
    ctx: Arc<Context>,
    csv: csv::Writer<AddressBalances>,
}

impl Balances {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("balances").await?;
        Ok(Self { ctx, csv })
    }
}

#[async_trait]
impl Exporter for Balances {
    #[tracing::instrument(skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting all balances");

        let response = self.ctx.cosmos.bank.balances(token.owner.clone()).await?;
        let balances: HashMap<String, String> = response
            .balances
            .into_iter()
            .map(|coin| (coin.denom, coin.amount))
            .collect();

        let assets = AddressBalances {
            address: token.owner.clone(),
            balances,
        };

        self.csv.write(assets).await?;

        tracing::info!("balance export finished");

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
