use std::collections::HashSet;

use async_trait::async_trait;

use crate::prelude::*;
use crate::queriers::soulbound::SoulboundToken;
use crate::{csv, Context};

use super::Exporter;

pub struct Patches {
    csv: csv::Writer<AddressPatch>,
    soulbound_token: SoulboundToken,
}

impl Patches {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("patches").await?;
        let soulbound_token = SoulboundToken::new(ctx.clone());

        Ok(Self {
            csv,
            soulbound_token,
        })
    }

    pub async fn all_tokens(&self) -> Result<HashSet<String>> {
        let tokens = self.soulbound_token.all_tokens().await?;
        Ok(tokens)
    }
}

#[async_trait]
impl Exporter for Patches {
    #[tracing::instrument(skip(self))]
    async fn export(&self, address: &str) -> Result<()> {
        tracing::info!("exporting soulbound patches");

        // let assets = AddressPatch {
        //     address: address.to_string(),
        //     patch_name,
        //     social_score,
        //     ranking,
        // };
        //
        // self.csv.write(assets).await?;

        tracing::info!("soulbound patches export finished");

        Ok(())
    }
}

pub struct AddressPatch {
    address: String,
    patch_name: String,
    social_score: u64,
    ranking: f64,
}

impl csv::Item for AddressPatch {
    fn header() -> csv::Header {
        vec!["address", "patch_name", "social_score", "ranking"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address.clone(),
            self.patch_name.clone(),
            self.social_score.to_string(),
            self.ranking.to_string(),
        ]]
    }
}
