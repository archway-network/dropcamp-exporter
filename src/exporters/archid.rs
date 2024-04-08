use async_trait::async_trait;

use crate::prelude::*;
use crate::{csv, queriers::archid::ArchIdRegistry, Context};

use super::Exporter;

pub struct ArchId {
    csv: csv::Writer<AddressNames>,
    archid: ArchIdRegistry,
}

impl ArchId {
    pub async fn create(ctx: Arc<Context>) -> Result<Self> {
        let csv = ctx.csv_writer("archid").await?;
        let archid = ArchIdRegistry::new(ctx);

        Ok(Self { csv, archid })
    }
}

#[async_trait]
impl Exporter for ArchId {
    #[tracing::instrument(skip(self))]
    async fn export(&self, address: &str) -> Result<()> {
        tracing::info!("exporting ArchID domains");

        let names = self.archid.resolve_domains(address.to_string()).await?;
        let assets = AddressNames {
            address: address.to_string(),
            names,
        };

        self.csv.write(assets).await?;

        tracing::info!("ArchID domains export finished");

        Ok(())
    }
}

pub struct AddressNames {
    address: String,
    names: Vec<String>,
}

impl csv::Item for AddressNames {
    fn header() -> csv::Header {
        vec!["address", "domains", "total"]
    }

    fn rows(self) -> Vec<csv::Row> {
        vec![vec![
            self.address.clone(),
            self.names.join(", "),
            self.names.len().to_string(),
        ]]
    }
}
