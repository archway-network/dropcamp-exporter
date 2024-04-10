use std::collections::HashMap;
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::util::BoxService;
use tower::{BoxError, Service, ServiceBuilder, ServiceExt};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct CoinPrice {
    pub usd: f64,
}

type PriceResponse = HashMap<String, CoinPrice>;

#[derive(Debug)]
pub struct CoinGeckoClient {
    url: Url,
    svc: Mutex<BoxService<reqwest::Request, reqwest::Response, BoxError>>,
}

impl CoinGeckoClient {
    pub fn builder(url: Url) -> Builder {
        Builder { url }
    }

    pub async fn price(&self, ids: &[&str]) -> Result<PriceResponse> {
        let endpoint = "/simple/price";
        let url = self
            .url
            .join(format!("{endpoint}?ids={}vs_currencies=usd", ids.join(",")).as_str())?;

        let mut svc = self.svc.lock().await;
        let client = svc.ready().await.map_err(|err| anyhow!(err))?;

        tracing::debug!("executing request");
        let request = reqwest::Request::new(reqwest::Method::GET, url);
        let response = client.call(request).await.map_err(|err| anyhow!(err))?;
        tracing::debug!(?response, "got response");

        let json = response.json::<PriceResponse>().await?;

        Ok(json)
    }
}

pub struct Builder {
    url: Url,
}

impl Builder {
    pub async fn build(self) -> Result<CoinGeckoClient> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        let svc = ServiceBuilder::new()
            .buffer(10)
            .concurrency_limit(5)
            .rate_limit(30, Duration::from_secs(60))
            .service(client)
            .boxed()
            .into();

        Ok(CoinGeckoClient { url: self.url, svc })
    }
}
