use anyhow::{bail, Result};

use clap::Parser;
use dropcamp_exporter::App;

use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::parse();
    let _guard = setup_logger(app.log_level)?;

    if let Err(e) = app.run().await {
        tracing::error!("execution failed: {}", e);
        bail!("aborted");
    };

    Ok(())
}

pub fn setup_logger(level: LevelFilter) -> Result<WorkerGuard> {
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
