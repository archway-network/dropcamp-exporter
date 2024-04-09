use async_trait::async_trait;
use futures::prelude::*;

use crate::prelude::*;

mod archid;
mod astrovault;
mod balances;
mod delegations;
mod liquid;
mod patches;

#[async_trait]
pub trait Exporter: Sync + Send {
    async fn export(&self, address: &str) -> Result<()>;
}

pub async fn run(ctx: Arc<Context>) -> Result<()> {
    tracing::info!("starting data export");

    ctx.create_output_folder()?;

    let patches_exporter = Box::new(patches::Patches::create(ctx.clone()).await?);
    let addresses = patches_exporter.all_tokens().await?;

    let exporters: Vec<Box<dyn Exporter>> = vec![
        patches_exporter,
        Box::new(balances::Balances::create(ctx.clone()).await?),
        Box::new(delegations::Delegations::create(ctx.clone()).await?),
        Box::new(archid::ArchId::create(ctx.clone()).await?),
        Box::new(liquid::LiquidFinance::create(ctx.clone()).await?),
        Box::new(astrovault::Astrovault::create(ctx.clone()).await?),
    ];

    let results = stream::iter(addresses.iter())
        .map(|address| {
            let tasks: Vec<_> = exporters
                .iter()
                .map(|exporter| exporter.export(address))
                .collect();
            future::join_all(tasks).map(|_| Ok(()))
        })
        .buffer_unordered(32)
        .try_collect::<Vec<_>>()
        .await?;

    tracing::info!("data export finished for {} addresses", results.len());

    Ok(())
}
