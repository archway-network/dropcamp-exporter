use std::path::PathBuf;

use clap::Parser;

use crate::exporters;
use crate::prelude::*;
use crate::queriers::soulbound::SoulboundToken;

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
    pub rpc_req_second: Option<u64>,

    /// Runs the operation on a specific block height.
    /// Otherwise, it will query the chain to get the latest block height.
    #[arg(long)]
    pub height: Option<u64>,

    /// Address for the soulbound token cw721 smart contract.
    #[arg(long)]
    pub soulbound_address: String,

    /// Address for the ArchID registry smart contract.
    #[arg(long)]
    pub archid_address: String,

    /// Address for the Liquid Finance cw20 smart contract.
    #[arg(long)]
    pub liquid_finance_address: String,

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
            .rpc(self.rpc_url.clone(), self.rpc_req_second)
            .height(self.height)
            .soulbound_address(self.soulbound_address.clone())
            .archid_address(self.archid_address.clone())
            .liquid_finance_address(self.liquid_finance_address.clone())
            .output(self.output.clone())
            .build()
            .await?;
        let ctx = Arc::new(ctx);

        let soulbound_token = SoulboundToken::new(ctx.clone());
        let addresses = soulbound_token.get_owners().await?;

        exporters::run(ctx, &addresses).await?;

        Ok(())
    }
}
