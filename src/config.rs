use anyhow::*;
use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Chain {
    pub id: String,
    pub denom: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Endpoint {
    pub url: Url,
    pub rate_limit: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub chain: Chain,
    pub rpc: Endpoint,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    chain: Option<Chain>,
    rpc: Option<Endpoint>,
}

impl ConfigBuilder {
    pub fn chain(mut self, id: String, denom: String) -> Self {
        self.chain = Some(Chain { id, denom });
        self
    }

    pub fn rpc(mut self, url: Url, rate_limit: Option<u64>) -> Self {
        self.rpc = Some(Endpoint { url, rate_limit });
        self
    }

    pub fn build(self) -> Result<Config> {
        let config = Config {
            chain: self.chain.ok_or(anyhow!("missing network in config"))?,
            rpc: self.rpc.ok_or(anyhow!("missing rpc in config"))?,
        };

        Ok(config)
    }
}
