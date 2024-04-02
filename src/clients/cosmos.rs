use std::sync::Arc;

use anyhow::Result;
use url::Url;

use super::rpc::RpcClient;

mod bank;
mod cosmwasm;
mod staking;

#[derive(Debug)]
pub struct CosmosClient {
    pub bank: bank::QueryClient,
    pub staking: staking::QueryClient,
    pub cosmwasm: cosmwasm::QueryClient,
}

impl CosmosClient {
    pub async fn new(url: Url, rate_limit: Option<u64>, height: Option<u64>) -> Result<Self> {
        let rpc = Arc::new(
            RpcClient::builder(url)
                .rate_limit(rate_limit)
                .height(height)
                .build()
                .await?,
        );

        let client = CosmosClient {
            bank: bank::QueryClient::new(rpc.clone()),
            staking: staking::QueryClient::new(rpc.clone()),
            cosmwasm: cosmwasm::QueryClient::new(rpc.clone()),
        };

        Ok(client)
    }
}
