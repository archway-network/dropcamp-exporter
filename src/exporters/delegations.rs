use std::collections::HashMap;

use crate::{csv, Context};
use anyhow::Result;
use async_trait::async_trait;

use super::Exporter;

pub struct Delegations {
    ctx: Context,
    csv: csv::Writer<ActiveDelegations>,
}

impl Delegations {
    pub async fn create(ctx: Context) -> Result<Self> {
        let csv = ctx.csv_writer("delegations").await?;
        Ok(Self { ctx, csv })
    }
}

#[async_trait]
impl Exporter for Delegations {
    #[tracing::instrument(skip(self))]
    async fn export(&self, address: &str) -> Result<()> {
        tracing::debug!("exporting delegations");

        let response = self
            .ctx
            .cosmos
            .staking
            .delegations(address.to_string())
            .await?;

        let delegations: HashMap<String, String> = response
            .delegation_responses
            .into_iter()
            .map(|delegations| {
                match (
                    delegations.delegation.map(|d| d.validator_address),
                    delegations.balance.map(|coin| coin.amount),
                ) {
                    (Some(validator), Some(amount)) => Some((validator, amount)),
                    _ => None,
                }
            })
            .flatten()
            .collect();

        let active_delegations = ActiveDelegations {
            address: address.to_string(),
            delegations,
        };

        self.csv.write(active_delegations).await?;

        Ok(())
    }
}

pub struct ActiveDelegations {
    address: String,
    delegations: HashMap<String, String>,
}

impl csv::Item for ActiveDelegations {
    fn header() -> csv::Header {
        vec!["address", "validator", "amount"]
    }

    fn rows(self) -> Vec<csv::Row> {
        let mut rows = vec![];
        for (validator, amount) in self.delegations {
            let row = vec![self.address.clone(), validator, amount];
            rows.push(row);
        }
        rows
    }
}
