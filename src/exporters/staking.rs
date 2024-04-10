use std::collections::HashMap;

use async_trait::async_trait;
use bigdecimal::BigDecimal;

use crate::prelude::*;
use crate::queriers::soulbound::TokenInfo;
use crate::{csv, Context};

use super::Exporter;

pub struct Staking {
    ctx: Arc<Context>,
    csv: csv::Writer<ActiveDelegations>,
}

impl Staking {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("staking").await?;
        Ok(Self { ctx, csv })
    }
}

#[async_trait]
impl Exporter for Staking {
    #[tracing::instrument(name = "staking::export", skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting delegations");

        let response = self
            .ctx
            .cosmos
            .staking
            .delegations(token.owner.clone())
            .await?;

        let delegations: HashMap<String, BigDecimal> = response
            .delegation_responses
            .into_iter()
            .filter_map(|delegations| {
                let validator = delegations.delegation.map(|d| d.validator_address);
                let amount = delegations
                    .balance
                    .and_then(|coin| to_bigdecimal(&coin.amount).ok());

                match (validator, amount) {
                    (Some(validator), Some(amount)) => Some((validator, amount)),
                    _ => None,
                }
            })
            .collect();

        let validators: Vec<String> = delegations
            .iter()
            .map(|(validator, _amount)| validator.clone())
            .collect();
        let delegated: BigDecimal = delegations
            .iter()
            .map(|(_validator, amount)| amount.with_scale(2))
            .sum();
        tracing::debug!(%delegated, ?validators, "total delegations");

        let delegated_score = delegated.to_f64().ok_or(anyhow!(
            "Failed to convert delegated amount to f64: {}",
            delegated
        ))?;
        let ranking = self
            .ctx
            .ranking
            .archway
            .activities
            .stake
            .ranking(delegated_score);

        let active_delegations = ActiveDelegations {
            address: token.owner.clone(),
            validators,
            delegated,
            ranking,
        };

        self.csv.write(active_delegations).await?;

        tracing::info!("delegations export finished");

        Ok(())
    }
}

pub struct ActiveDelegations {
    address: String,
    validators: Vec<String>,
    delegated: BigDecimal,
    ranking: f32,
}

impl csv::Item for ActiveDelegations {
    fn header() -> csv::Header {
        vec!["address", "ranking", "delegated", "validators"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address.clone(),
            format!("{:.2}", self.ranking),
            self.delegated.to_string(),
            self.validators.join(","),
        ]]
    }
}
