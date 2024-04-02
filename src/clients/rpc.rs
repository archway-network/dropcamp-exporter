use anyhow::{anyhow, Result};
use cosmos_sdk_proto::prost::Message;
use tendermint::block::Height;
use tendermint_rpc::{client::CompatMode, Client, HttpClient, Url};

use crate::block::Block;

/// A RPC client wrapper that queries ProtoBuf
/// encoded message for a single block height.
#[derive(Debug)]
pub struct RpcClient {
    inner: HttpClient,
    block: Block,
}

impl RpcClient {
    pub fn builder(url: Url) -> Builder {
        Builder {
            url,
            rate_limit: None,
            height: None,
        }
    }

    #[tracing::instrument(fields(height = self.block.height.value()), skip(self, data))]
    pub async fn request<T, R>(&self, service: &str, method: &str, data: T) -> anyhow::Result<R>
    where
        T: Message + Default,
        R: Message + Default,
    {
        tracing::debug!(?data, "request");

        let path = format!("/{service}/{method}");
        let response = self
            .inner
            .abci_query(
                Some(path),
                data.encode_to_vec(),
                Some(self.block.height),
                false,
            )
            .await?;

        let response: R = R::decode(&*response.value)?;
        tracing::debug!(?response, "response");

        Ok(response)
    }
}

pub struct Builder {
    url: Url,
    rate_limit: Option<u64>,
    height: Option<u64>,
}

impl Builder {
    pub fn rate_limit(mut self, rate_limit: Option<u64>) -> Self {
        self.rate_limit = rate_limit;
        self
    }

    pub fn height(mut self, height: Option<u64>) -> Self {
        self.height = height;
        self
    }

    pub async fn build(self) -> Result<RpcClient> {
        let url: TmRpcUrl = self.url.clone().try_into()?;
        let client = HttpClient::builder(url.try_into()?)
            .compat_mode(CompatMode::V0_37)
            .build()?;

        let block = self.get_block(&client).await?;
        tracing::info!(block = ?block, "creating rpc client for block");

        Ok(RpcClient { inner, block })
    }

    async fn get_block(&self, client: &HttpClient) -> Result<Block> {
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
