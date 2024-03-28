use std::sync::Arc;

use crate::clients::CosmosClient;
use crate::config::Config;

#[derive(Clone, Debug)]
pub struct Context {
    pub config: Config,
    pub cosmos: Arc<CosmosClient>,
}

impl Context {
    pub async fn build(config: Config, height: Option<u64>) -> anyhow::Result<Self> {
        let cosmos = CosmosClient::builder()
            .url(config.rpc.url.clone().try_into()?)
            .rate_limit(config.rpc.rate_limit)
            .height(height)
            .build()
            .await?;

        let ctx = Self {
            config,
            cosmos: Arc::new(cosmos),
        };

        Ok(ctx)
    }
}
