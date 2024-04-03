use std::{any::type_name, collections::HashSet};

use anyhow::Result;
use async_trait::async_trait;
use futures::{future, stream, StreamExt, TryStreamExt};

use crate::Context;

mod archid;
mod balances;
mod delegations;

#[async_trait]
pub trait Exporter: Sync + Send {
    async fn export(&self, address: &str) -> Result<()>;
}

pub async fn run(ctx: &Context, addresses: &HashSet<String>) -> Result<()> {
    tracing::info!("running exporters");

    let exporters: Vec<Box<dyn Exporter>> = vec![
        Box::new(balances::Balances::create(ctx.clone()).await?),
        Box::new(delegations::Delegations::create(ctx.clone()).await?),
        Box::new(archid::ArchId::create(ctx.clone()).await?),
    ];

    let tasks = exporters
        .iter()
        .map(|exporter| export(exporter, &addresses))
        .collect::<Vec<_>>();

    future::try_join_all(tasks).await?;

    Ok(())
}

#[tracing::instrument(skip_all, fields(exporter = type_name::<T>()))]
async fn export<T>(exporter: &Box<T>, addresses: &HashSet<String>) -> Result<()>
where
    T: Exporter + ?Sized,
{
    tracing::info!("starting data export");

    stream::iter(addresses.iter())
        .map(|address| exporter.export(address.as_str()))
        .buffer_unordered(10)
        .try_collect::<Vec<_>>()
        .await?;

    tracing::info!("data export finished");

    Ok(())
}
