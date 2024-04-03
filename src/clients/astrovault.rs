use std::time::Duration;

use anyhow::{anyhow, Result};
use reqwest::header;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::limit::RateLimitLayer;
use tower::util::BoxService;
use tower::{BoxError, Service, ServiceBuilder, ServiceExt};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct WalletStats {
    pub has_lpd: bool,
    pub has_traded: bool,
}

#[derive(Debug, Deserialize)]
pub struct WalletTvl {
    pub address: String,
    pub tvl: f64,
}

pub struct AstrovaultClient {
    url: Url,
    svc: Mutex<BoxService<reqwest::Request, reqwest::Response, BoxError>>,
}

impl AstrovaultClient {
    pub fn builder(url: Url) -> Builder {
        Builder {
            url,
            req_second: None,
            api_key: None,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn stats(&self, address: &str) -> Result<WalletStats> {
        tracing::debug!("get wallet stats");
        self.request("/wallet/stats", address).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn tvl(&self, address: &str) -> Result<WalletTvl> {
        tracing::debug!("get wallet tlv");
        self.request("/wallet/tvl", address).await
    }

    async fn request<R>(&self, path: &str, address: &str) -> Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let url = self
            .url
            .join(format!("{path}?address={address}").as_str())?;

        let mut svc = self.svc.lock().await;
        let client = svc.ready().await.map_err(|err| anyhow!(err))?;

        let request = reqwest::Request::new(reqwest::Method::GET, url);
        let response = client.call(request).await.map_err(|err| anyhow!(err))?;
        let body = response.json::<R>().await?;

        Ok(body)
    }
}

pub struct Builder {
    url: Url,
    req_second: Option<u64>,
    api_key: Option<String>,
}

impl Builder {
    pub fn req_second(mut self, req_second: Option<u64>) -> Self {
        self.req_second = req_second;
        self
    }

    pub fn api_key(mut self, api_key: Option<String>) -> Self {
        self.api_key = api_key;
        self
    }

    pub async fn build(self) -> Result<AstrovaultClient> {
        let mut headers = header::HeaderMap::new();

        if let Some(api_key) = self.api_key {
            let mut auth_value = header::HeaderValue::from_str(api_key.as_str())?;
            auth_value.set_sensitive(true);
            headers.insert("x-api-key", auth_value);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        let svc = ServiceBuilder::new()
            .buffer(100)
            .concurrency_limit(50)
            .option_layer(
                self.req_second
                    .map(|num| RateLimitLayer::new(num, Duration::from_secs(1))),
            )
            .service(client)
            .boxed()
            .into();

        Ok(AstrovaultClient { url: self.url, svc })
    }
}
