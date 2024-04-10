use async_trait::async_trait;

use crate::{csv, prelude::*, queriers::soulbound::TokenInfo};

use super::Exporter;

pub struct Astrovault {
    ctx: Arc<Context>,
    csv: csv::Writer<AstrovaultPosition>,
}

impl Astrovault {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("astrovault").await?;
        Ok(Self { ctx, csv })
    }
}

#[async_trait]
impl Exporter for Astrovault {
    #[tracing::instrument(name = "astrovault::export", skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting Astrovault stats and tvl");

        let stats = self.ctx.astrovault.stats(token.owner.as_str()).await?;
        let tvl = self.ctx.astrovault.tvl(token.owner.as_str()).await?;

        let ranking = self
            .ctx
            .ranking
            .ecosystem
            .activities
            .astrovault
            .ranking(tvl.tvl);

        let position = AstrovaultPosition {
            address: token.owner.clone(),
            has_lpd: stats.has_lpd,
            has_traded: stats.has_traded,
            tvl: tvl.tvl,
            ranking,
        };

        self.csv.write(position).await?;

        tracing::info!("Astrovault stats and tvl export finished");

        Ok(())
    }
}

pub struct AstrovaultPosition {
    address: String,
    has_lpd: bool,
    has_traded: bool,
    tvl: f64,
    ranking: f32,
}

impl csv::Item for AstrovaultPosition {
    fn header() -> csv::Header {
        vec!["address", "ranking", "has_lpd", "has_traded", "tvl"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address,
            format!("{:.2}", self.ranking),
            self.has_lpd.to_string(),
            self.has_traded.to_string(),
            self.tvl.to_string(),
        ]]
    }
}
