use std::collections::HashMap;
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::util::BoxService;
use tower::{BoxError, Service, ServiceBuilder, ServiceExt};
use url::Url;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct CoinPrice {
    pub usd: f64,
}

type PriceResponse = HashMap<String, CoinPrice>;

#[derive(Debug)]
pub struct CoinGeckoClient {
    url: Url,
    svc: Mutex<BoxService<reqwest::Request, reqwest::Response, BoxError>>,
    price_cache: Mutex<PriceResponse>,
}

impl CoinGeckoClient {
    pub fn new(
        url: Url,
        svc: Mutex<BoxService<reqwest::Request, reqwest::Response, BoxError>>,
    ) -> Self {
        Self {
            url,
            svc,
            price_cache: HashMap::new().into(),
        }
    }

    pub fn builder(url: Url) -> Builder {
        Builder { url }
    }

    pub async fn price(&self, ids: Vec<&str>) -> Result<PriceResponse> {
        tracing::debug!(?ids, "fetching coin prices");

        if ids.is_empty() {
            return Ok(PriceResponse::default());
        }

        let mut cache = self.price_cache.lock().await;

        let query_ids: Vec<&str> = ids
            .iter()
            .filter(|&id| !cache.contains_key(*id))
            .copied()
            .collect();

        if !query_ids.is_empty() {
            let endpoint = format!(
                "/api/v3/simple/price?ids={}&vs_currencies=usd&precision=full",
                query_ids.as_slice().join(",")
            );
            let response: PriceResponse = self.request(endpoint.as_str()).await?;
            cache.extend(response.clone());
        }

        let prices = ids
            .into_iter()
            .map(|id| {
                cache
                    .get(id)
                    .as_deref()
                    .map(|&price| (id.to_string(), price))
                    .ok_or(anyhow!("missing price for {}", id))
            })
            .collect::<Result<PriceResponse>>()?;

        Ok(prices)
    }

    #[tracing::instrument(skip(self))]
    async fn request<R>(&self, endpoint: &str) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let mut svc = self.svc.lock().await;
        let client = svc.ready().await.map_err(|err| anyhow!(err))?;

        let url = self.url.join(endpoint)?;
        let request = reqwest::Request::new(reqwest::Method::GET, url);
        tracing::debug!(?request, "executing request");

        let response = client.call(request).await.map_err(|err| anyhow!(err))?;
        tracing::debug!(?response, "got response");

        let json = response.json::<R>().await?;

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

        Ok(CoinGeckoClient::new(self.url, svc))
    }
}
