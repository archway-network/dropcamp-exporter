use std::sync::Arc;

use anyhow::*;
use tendermint::block::Height;
use tendermint_rpc::{client::CompatMode, Client, HttpClient, Url};

use crate::block::Block;

use super::rpc::RpcClient;

mod bank;
mod cosmwasm;
mod staking;

#[derive(Debug)]
pub struct CosmosClient {
    pub block: Block,
    pub bank: bank::QueryClient,
    pub staking: staking::QueryClient,
    pub cosmwasm: cosmwasm::QueryClient,
}

impl CosmosClient {
    fn new(block: Block, inner: HttpClient) -> Self {
        let rpc = Arc::new(RpcClient::new(inner, block.height));

        Self {
            block,
            bank: bank::QueryClient::new(rpc.clone()),
            staking: staking::QueryClient::new(rpc.clone()),
            cosmwasm: cosmwasm::QueryClient::new(rpc.clone()),
        }
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Default)]
pub struct Builder {
    url: Option<Url>,
    rate_limit: Option<u64>,
    height: Option<u64>,
}

impl Builder {
    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    pub fn rate_limit(mut self, rate_limit: Option<u64>) -> Self {
        self.rate_limit = rate_limit;
        self
    }

    pub fn height(mut self, height: Option<u64>) -> Self {
        self.height = height;
        self
    }

    pub async fn build(self) -> anyhow::Result<CosmosClient> {
        let url = self.url.clone().ok_or_else(|| anyhow!("missing RPC URL"))?;
        let inner = HttpClient::builder(url.try_into()?)
            .compat_mode(CompatMode::V0_37)
            .build()?;

        let block = self.get_block(&inner).await?;
        tracing::info!(block = ?block, "creating rpc client for block");

        Ok(CosmosClient::new(block, inner))
    }

    async fn get_block(self, client: &HttpClient) -> Result<Block> {
        let block = match self.height {
            Some(height) => {
                tracing::debug!(%height, "using provided block height");
                let height: Height = height.try_into()?;
                client.block(height).await?
            }
            None => {
                tracing::debug!("querying the chain for the latest block height");
                client.latest_block().await?
            }
        };

        Ok(block.block.header.try_into()?)
    }
}
