use std::collections::HashMap;

use async_trait::async_trait;

use crate::coin::Coin;
use crate::prelude::*;
use crate::queriers::soulbound::TokenInfo;
use crate::{csv, Context};

use super::Exporter;

pub struct Ibc {
    ctx: Arc<Context>,
    csv: csv::Writer<AddressBalances>,
}

impl Ibc {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("ibc").await?;
        Ok(Self { ctx, csv })
    }

    async fn calculate_balances(&self, address: String) -> Result<Vec<Balance>> {
        let response = self.ctx.cosmos.bank.balances(address).await?;
        let coins: Vec<Coin> = response
            .balances
            .into_iter()
            .flat_map(|coin| self.ctx.token_map.create_coin(coin))
            .flatten()
            .collect();
        tracing::debug!(?coins, "mapped tokens in wallet");

        let coingecko_ids: Vec<&str> = coins
            .iter()
            .flat_map(|coin| coin.coingecko_id.as_deref())
            .collect();
        let prices = self.ctx.coingecko.price(coingecko_ids).await?;
        let balances = coins
            .into_iter()
            .flat_map(|coin| {
                coin.coingecko_id.as_ref().and_then(|id| {
                    prices.get(id).map(|price| {
                        coin.total_value(price.usd).map(|total_value| Balance {
                            balance: coin.clone(),
                            total_value,
                        })
                    })
                })
            })
            .flatten()
            .collect();
        tracing::debug!(?balances, "token balances in usd");

        Ok(balances)
    }
}

#[async_trait]
impl Exporter for Ibc {
    #[tracing::instrument(name = "balances::export", skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting all token balances");

        let balances = self.calculate_balances(token.owner.clone()).await?;
        let usd = balances.iter().map(|balance| balance.total_value).sum();
        let ranking = self.ctx.ranking.archway.activities.ibc.ranking(usd);

        let assets = AddressBalances {
            address: token.owner.clone(),
            balances,
            usd,
            ranking,
        };

        self.csv.write(assets).await?;

        tracing::info!("token balances export finished");

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Balance {
    pub balance: Coin,
    pub total_value: f64,
}

pub struct AddressBalances {
    address: String,
    balances: Vec<Balance>,
    usd: f64,
    ranking: f32,
}

impl csv::Item for AddressBalances {
    fn header() -> csv::Header {
        vec!["address", "ranking", "usd", "balances"]
    }

    fn rows(self) -> Vec<csv::Row> {
        let balances: Vec<String> = self
            .balances
            .iter()
            .map(|balance| balance.balance.with_scale(2).to_string())
            .collect();

        vec![vec![
            self.address.clone(),
            format!("{:.2}", self.ranking),
            format!("{:.2}", self.usd),
            balances.join(","),
        ]]
    }
}
