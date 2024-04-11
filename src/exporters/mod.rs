use async_trait::async_trait;
use futures::prelude::*;

use crate::{prelude::*, queriers::soulbound::TokenInfo};

mod archid;
mod astrovault;
mod ibc;
mod liquid;
mod socials;
mod staking;

#[async_trait]
pub trait Exporter: Sync + Send {
    async fn export(&self, token: &TokenInfo) -> Result<()>;
}

pub async fn run(ctx: Arc<Context>) -> Result<()> {
    tracing::info!("starting data export");

    ctx.create_output_folder()?;

    let socials_exporter = socials::Socials::create(ctx.clone()).await?;
    let tokens = socials_exporter.all_tokens().await?;

    let exporters: Vec<Box<dyn Exporter>> = vec![
        Box::new(socials_exporter),
        Box::new(ibc::Ibc::create(ctx.clone()).await?),
        Box::new(staking::Staking::create(ctx.clone()).await?),
        Box::new(archid::ArchId::create(ctx.clone()).await?),
        Box::new(liquid::LiquidFinance::create(ctx.clone()).await?),
        Box::new(astrovault::Astrovault::create(ctx.clone()).await?),
    ];

    stream::iter(tokens.iter())
        .flat_map(|token| stream::iter(exporters.iter()).map(|exporter| exporter.export(token)))
        .buffer_unordered(32)
        .try_collect::<Vec<_>>()
        .await?;

    tracing::info!("data export finished");

    Ok(())
}
