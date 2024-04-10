use std::path::PathBuf;

use clap::Parser;

use crate::exporters;
use crate::prelude::*;

use url::Url;

const RPC_URL: &str = "https://rpc.mainnet.archway.io:443";
const COINGECKO_URL: &str = "https://api.coingecko.com/api/v3";

const RANKING_FILE: &str = "ranking.toml";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct App {
    /// Url for the RPC endpoint.
    #[arg(long, default_value = RPC_URL)]
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

    /// Url for the Astrovault liquidity pools API.
    #[arg(long)]
    pub astrovault_url: Url,

    /// Limits the number of requests per second to the Astrovault API.
    #[arg(long)]
    pub astrovault_req_second: Option<u64>,

    /// API key for the Astrovault API.
    #[arg(long)]
    pub astrovault_api_key: Option<String>,

    /// Url for the CoinGecko API.
    #[arg(long, default_value = COINGECKO_URL)]
    pub coingecko_url: Url,

    /// Path for the ranking config file.
    #[arg(long, default_value = RANKING_FILE)]
    pub ranking: PathBuf,

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
            .rpc(self.rpc_url.clone(), self.rpc_req_second)
            .height(self.height)
            .soulbound_address(self.soulbound_address.clone())
            .archid_address(self.archid_address.clone())
            .liquid_finance_address(self.liquid_finance_address.clone())
            .astrovault(
                self.astrovault_url.clone(),
                self.astrovault_req_second,
                self.astrovault_api_key.clone(),
            )
            .coingecko(self.coingecko_url.clone())
            .ranking_path(self.ranking.clone())
            .output(self.output.clone())
            .build()
            .await?;
        let ctx = Arc::new(ctx);

        exporters::run(ctx).await?;

        Ok(())
    }
}
