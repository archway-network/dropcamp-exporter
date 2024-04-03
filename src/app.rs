use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::queriers::soulbound::SoulboundToken;
use crate::Context;

use url::Url;

const RPC: &str = "https://rpc.mainnet.archway.io:443";

const CHAIN_ID: &str = "archway-1";

const DENOM: &str = "aarch";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct App {
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

    /// Address for the soulbound token smart contract.
    #[arg(long)]
    pub soulbound_address: String,

    /// Directory path to output the CSV files.
    #[arg(short, long)]
    pub output: PathBuf,

    /// Sets the log level.
    #[arg(long, value_name = "LEVEL", default_value = "info")]
    pub log_level: tracing::metadata::LevelFilter,
}

impl App {
    pub async fn run(&self) -> Result<()> {
        let ctx = Context::builder()
            .chain(self.chain_id.clone(), self.denom.clone())
            .rpc(self.rpc_url.clone(), self.rpc_rate_limit)
            .height(self.height)
            .soulbound_address(self.soulbound_address.clone())
            .output(self.output.clone())
            .build()
            .await?;

        let soulbound_token = SoulboundToken::new(ctx.clone());
        let _owners = soulbound_token.get_owners().await?;

        Ok(())
    }
}
