use std::path::PathBuf;

use anyhow::*;

use clap::{command, Parser};
use url::Url;

use crate::config::Config;

const RPC: &str = "https://rpc.mainnet.archway.io:443";

const CHAIN_ID: &str = "archway-1";

const DENOM: &str = "aarch";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Exporter {
    /// ID of the chain.
    #[arg(long, default_value = CHAIN_ID)]
    pub chain_id: String,

    /// Denom for the chain token.
    #[arg(long, default_value = DENOM)]
    pub denom: String,

    /// Url for the RPC endpoint.
    #[arg(long, default_value = RPC)]
    pub rpc_url: Url,

    /// Limits the number of requests per second to the RPC endpoint.
    #[arg(long)]
    pub rpc_rate_limit: Option<u64>,

    /// Runs the operation on a specific block height.
    /// Otherwise, it will query the chain to get the latest block height.
    #[arg(long)]
    pub height: Option<u64>,

    /// Directory path to output the CSV files.
    #[arg(short, long)]
    pub output: PathBuf,

    /// Sets the log level.
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    pub log_level: tracing::metadata::LevelFilter,
}

impl Exporter {
    pub async fn execute(self) -> Result<()> {
        let config = self.build_config()?;
        Ok(())
    }

    fn build_config(&self) -> Result<Config> {
        Config::builder()
            .chain(self.chain_id.clone(), self.denom.clone())
            .rpc(self.rpc_url.clone(), self.rpc_rate_limit)
            .build()
    }
}
