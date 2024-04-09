use async_trait::async_trait;
use futures::prelude::*;

use crate::{prelude::*, queriers::soulbound::TokenInfo};

mod archid;
mod astrovault;
mod balances;
mod delegations;
mod liquid;
mod patches;

#[async_trait]
pub trait Exporter: Sync + Send {
    async fn export(&self, token: &TokenInfo) -> Result<()>;
}

pub async fn run(ctx: Arc<Context>) -> Result<()> {
    tracing::info!("starting data export");

    ctx.create_output_folder()?;

    let patches_exporter = patches::Patches::create(ctx.clone()).await?;
    let tokens = patches_exporter.all_tokens().await?;

    let exporters: Vec<Box<dyn Exporter>> = vec![
        Box::new(patches_exporter),
        Box::new(balances::Balances::create(ctx.clone()).await?),
        Box::new(delegations::Delegations::create(ctx.clone()).await?),
        Box::new(archid::ArchId::create(ctx.clone()).await?),
        Box::new(liquid::LiquidFinance::create(ctx.clone()).await?),
        Box::new(astrovault::Astrovault::create(ctx.clone()).await?),
    ];

    let results = stream::iter(tokens.iter())
        .map(|token| {
            let tasks: Vec<_> = exporters
                .iter()
                .map(|exporter| exporter.export(token))
                .collect();
            future::join_all(tasks).map(|_| Ok(()))
        })
        .buffer_unordered(32)
        .try_collect::<Vec<_>>()
        .await?;

    tracing::info!("data export finished for {} addresses", results.len());

    Ok(())
}
