use anyhow::*;
use clap::Parser;

use dropcamp_exporter::exporter::Exporter;

use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let exporter = Exporter::parse();

    let _guard = setup_logger(exporter.log_level)?;

    if let Err(e) = exporter.execute().await {
        tracing::error!("execution failed: {}", e);
        bail!("aborted");
    };

    Ok(())
}

pub fn setup_logger(level: LevelFilter) -> anyhow::Result<WorkerGuard> {
    let buffer = std::io::stderr();
    let (writer, _guard) = tracing_appender::non_blocking(buffer);
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_writer(writer)
        .finish()
        .with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(_guard)
}
