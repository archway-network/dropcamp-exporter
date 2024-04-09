use std::collections::HashMap;

use async_trait::async_trait;

use crate::prelude::*;
use crate::queriers::soulbound::TokenInfo;
use crate::{csv, Context};

use super::Exporter;

pub struct Delegations {
    ctx: Arc<Context>,
    csv: csv::Writer<ActiveDelegations>,
}

impl Delegations {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("delegations").await?;
        Ok(Self { ctx, csv })
    }
}

#[async_trait]
impl Exporter for Delegations {
    #[tracing::instrument(skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting delegations");

        let response = self
            .ctx
            .cosmos
            .staking
            .delegations(token.owner.clone())
            .await?;

        let delegations: HashMap<String, String> = response
            .delegation_responses
            .into_iter()
            .filter_map(|delegations| {
                match (
                    delegations.delegation.map(|d| d.validator_address),
                    delegations.balance.map(|coin| coin.amount),
                ) {
                    (Some(validator), Some(amount)) => Some((validator, amount)),
                    _ => None,
                }
            })
            .collect();

        let active_delegations = ActiveDelegations {
            address: token.owner.clone(),
            delegations,
        };

        self.csv.write(active_delegations).await?;

        tracing::info!("delegations export finished");

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
