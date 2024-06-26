use async_trait::async_trait;

use crate::prelude::*;
use crate::queriers::soulbound::{SoulboundToken, TokenInfo};
use crate::{csv, Context};

use super::Exporter;

pub struct Socials {
    ctx: Arc<Context>,
    csv: csv::Writer<AddressSocialPatch>,
    soulbound_token: SoulboundToken,
}

impl Socials {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("socials").await?;
        let soulbound_token = SoulboundToken::new(ctx.clone());

        Ok(Self {
            csv,
            ctx,
            soulbound_token,
        })
    }

    pub async fn all_tokens(&self) -> Result<Vec<TokenInfo>> {
        let tokens = self.soulbound_token.all_tokens().await?;
        Ok(tokens)
    }
}

#[async_trait]
impl Exporter for Socials {
    #[tracing::instrument(name = "social::export", skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting soulbound patches");

        let ranking = self.ctx.ranking.social.weighted_ranking(token.social_score);

        let assets = AddressSocialPatch {
            address: token.owner.clone(),
            patch_name: token.name.clone(),
            social_score: token.social_score,
            ranking,
        };

        self.csv.write(assets).await?;

        tracing::info!("soulbound patches export finished");

        Ok(())
    }
}

pub struct AddressSocialPatch {
    address: String,
    patch_name: String,
    social_score: u16,
    ranking: f32,
}

impl csv::Item for AddressSocialPatch {
    fn header() -> csv::Header {
        vec!["address", "ranking", "patch_name", "social_score"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address.clone(),
            format!("{:.2}", self.ranking),
            self.patch_name.clone(),
            self.social_score.to_string(),
        ]]
    }
}
