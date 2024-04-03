use std::collections::HashSet;

use async_trait::async_trait;
use futures::{future, stream, StreamExt, TryStreamExt};

use crate::prelude::*;

mod archid;
mod astrovault;
mod balances;
mod delegations;
mod liquid;

#[async_trait]
pub trait Exporter: Sync + Send {
    async fn export(&self, address: &str) -> Result<()>;
}

pub async fn run(ctx: Arc<Context>, addresses: &HashSet<String>) -> Result<()> {
    tracing::info!("starting data export");

    ctx.create_output_folder()?;

    let exporters: Vec<Box<dyn Exporter>> = vec![
        Box::new(balances::Balances::create(ctx.clone()).await?),
        Box::new(delegations::Delegations::create(ctx.clone()).await?),
        Box::new(archid::ArchId::create(ctx.clone()).await?),
        Box::new(liquid::LiquidFinance::create(ctx.clone()).await?),
        Box::new(astrovault::Astrovault::create(ctx.clone()).await?),
    ];

    let tasks = exporters
        .iter()
        .map(|exporter| export(exporter, addresses))
        .collect::<Vec<_>>();

    future::try_join_all(tasks).await?;

    tracing::info!("data export finished");

    Ok(())
}

#[allow(clippy::borrowed_box)]
#[tracing::instrument(skip_all)]
async fn export<T>(exporter: &Box<T>, addresses: &HashSet<String>) -> Result<()>
where
    T: Exporter + ?Sized,
{
    stream::iter(addresses.iter())
        .map(|address| exporter.export(address.as_str()))
        .buffer_unordered(10)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(())
}
