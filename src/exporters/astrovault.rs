use async_trait::async_trait;

use crate::{csv, prelude::*};

use super::Exporter;

pub struct Astrovault {
    csv: csv::Writer<AstrovaultPosition>,
    ctx: Arc<Context>,
}

impl Astrovault {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("astrovault").await?;

        Ok(Self { csv, ctx })
    }
}

#[async_trait]
impl Exporter for Astrovault {
    #[tracing::instrument(skip(self))]
    async fn export(&self, address: &str) -> Result<()> {
        tracing::info!("exporting Astrovault stats and tvl");

        let stats = self.ctx.astrovault.stats(address).await?;
        let tvl = self.ctx.astrovault.tvl(address).await?;

        let position = AstrovaultPosition {
            address: address.to_string(),
            has_lpd: stats.has_lpd,
            has_traded: stats.has_traded,
            tvl: tvl.tvl,
        };

        self.csv.write(position).await?;

        Ok(())
    }
}

pub struct AstrovaultPosition {
    address: String,
    has_lpd: bool,
    has_traded: bool,
    tvl: f64,
}

impl csv::Item for AstrovaultPosition {
    fn header() -> csv::Header {
        vec!["address", "has_lpd", "has_traded", "tvl"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address,
            self.has_lpd.to_string(),
            self.has_traded.to_string(),
            self.tvl.to_string(),
        ]]
    }
}
