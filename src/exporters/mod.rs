use std::collections::HashSet;

use anyhow::Result;
use async_trait::async_trait;
use futures::future;

use crate::Context;

mod assets;

#[async_trait]
pub trait Exporter: Sync + Send {
    const NAME: &'static str;

    async fn export(&self, address: &str) -> Result<()>;
}

pub async fn run(ctx: &Context, addresses: &HashSet<String>) -> Result<()> {
    tracing::info!("running exporters");

    let assets = assets::AssetsExporter::create(ctx.clone()).await?;
    let exporters = vec![assets];

    let tasks = exporters
        .iter()
        .map(|exporter| export(exporter, &addresses))
        .collect::<Vec<_>>();

    future::try_join_all(tasks).await?;

    Ok(())
}

#[tracing::instrument(skip_all, fields(exporter = T::NAME))]
async fn export<T>(exporter: &T, addresses: &HashSet<String>) -> Result<()>
where
    T: Exporter,
{
    tracing::info!("starting data export");

    let tasks = addresses
        .iter()
        .map(|address| exporter.export(address.as_str()))
        .collect::<Vec<_>>();

    future::try_join_all(tasks).await?;

    tracing::info!("data export finished");

    Ok(())
}
