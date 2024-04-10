use async_trait::async_trait;

use crate::prelude::*;
use crate::queriers::soulbound::TokenInfo;
use crate::{csv, queriers::archid::ArchIdRegistry, Context};

use super::Exporter;

pub struct ArchId {
    ctx: Arc<Context>,
    csv: csv::Writer<AddressNames>,
    archid: ArchIdRegistry,
}

impl ArchId {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("archid").await?;
        let archid = ArchIdRegistry::new(ctx.clone());

        Ok(Self { ctx, csv, archid })
    }
}

#[async_trait]
impl Exporter for ArchId {
    #[tracing::instrument(skip_all, fields(address = token.owner))]
    async fn export(&self, token: &TokenInfo) -> Result<()> {
        tracing::info!("exporting ArchID domains");

        let names = self.archid.resolve_domains(token.owner.clone()).await?;
        let ranking = self
            .ctx
            .ranking
            .ecosystem
            .activities
            .archid
            .ranking(names.len() as f64);

        let assets = AddressNames {
            address: token.owner.clone(),
            names,
            ranking,
        };

        self.csv.write(assets).await?;

        tracing::info!("ArchID domains export finished");

        Ok(())
    }
}

pub struct AddressNames {
    address: String,
    names: Vec<String>,
    ranking: f32,
}

impl csv::Item for AddressNames {
    fn header() -> csv::Header {
        vec!["address", "ranking", "domains", "names"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address.clone(),
            format!("{:.2}", self.ranking),
            self.names.len().to_string(),
            self.names.join(", "),
        ]]
    }
}
