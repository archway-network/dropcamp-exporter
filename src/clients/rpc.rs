use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

use anyhow::{anyhow, Result};
use cosmos_sdk_proto::prost::Message;
use futures::prelude::future::{BoxFuture, FutureExt};
use tendermint::block::Height;
use tendermint_rpc::endpoint::abci_query;
use tendermint_rpc::{
    client::CompatMode, Client, Error as TmRpcError, HttpClient, Url as TmRpcUrl,
};
use tokio::sync::Mutex;
use tower::limit::RateLimitLayer;
use tower::util::BoxService;
use tower::{BoxError, Service, ServiceBuilder, ServiceExt};
use url::Url;

use crate::block::Block;

/// A RPC client wrapper that queries ProtoBuf
/// encoded message for a single block height.
#[derive(Debug)]
pub struct RpcClient {
    svc: Mutex<BoxService<abci_query::Request, abci_query::Response, BoxError>>,
    block: Block,
}

impl RpcClient {
    pub fn builder(url: Url) -> Builder {
        Builder {
            url,
            req_second: None,
            height: None,
        }
    }

    #[tracing::instrument(fields(height = self.block.height.value()), skip(self, data))]
    pub async fn request<T, R>(&self, service: &str, method: &str, data: T) -> Result<R>
    where
        T: Message + Default,
        R: Message + Default,
    {
        tracing::debug!(?data, "request");

        let path = format!("/{service}/{method}");
        let request = abci_query::Request::new(
            Some(path),
            data.encode_to_vec(),
            Some(self.block.height),
            false,
        );

        let mut svc = self.svc.lock().await;
        let client = svc.ready().await.map_err(|err| anyhow!(err))?;
        let response = client.call(request).await.map_err(|err| anyhow!(err))?;

        let response: R = R::decode(&*response.response.value)?;
        tracing::debug!(?response, "response");

        Ok(response)
    }
}

pub struct Builder {
    url: Url,
    req_second: Option<u64>,
    height: Option<u64>,
}

impl Builder {
    pub fn req_second(mut self, req_second: Option<u64>) -> Self {
        self.req_second = req_second;
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

        let svc = ServiceBuilder::new()
            .buffer(100)
            .concurrency_limit(50)
            .option_layer(
                self.req_second
                    .map(|num| RateLimitLayer::new(num, Duration::from_secs(1))),
            )
            .service(HttpClientWrapper::new(client))
            .boxed()
            .into();

        Ok(RpcClient { svc, block })
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

        block.block.header.try_into()
    }
}

#[derive(Debug, Clone)]
pub struct HttpClientWrapper {
    client: Arc<HttpClient>,
}

impl HttpClientWrapper {
    pub fn new(client: HttpClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
}

impl Service<abci_query::Request> for HttpClientWrapper {
    type Response = abci_query::Response;
    type Error = TmRpcError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&mut self, req: abci_query::Request) -> Self::Future {
        let client = self.client.clone();
        async move { client.perform(req).await }.boxed()
    }

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
