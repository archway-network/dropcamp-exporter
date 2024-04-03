use std::{path::PathBuf, sync::Arc};

use anyhow::*;
use url::Url;

use crate::{clients::CosmosClient, csv};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chain {
    pub id: String,
    pub denom: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Endpoint {
    pub url: Url,
    pub rate_limit: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub chain: Chain,
    pub soulbound_address: String,
    pub archid_address: String,
    pub liquid_finance_address: String,
    pub cosmos: Arc<CosmosClient>,
    output: PathBuf,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::default()
    }

    pub async fn csv_writer<T>(&self, name: &str) -> Result<csv::Writer<T>>
    where
        T: csv::Item,
    {
        let path = self.output.join(name).with_extension("csv");
        csv::Writer::create(path).await
    }
}

#[derive(Default)]
pub struct ContextBuilder {
    chain: Option<Chain>,
    rpc: Option<Endpoint>,
    height: Option<u64>,
    soulbound_address: Option<String>,
    archid_address: Option<String>,
    liquid_finance_address: Option<String>,
    output: Option<PathBuf>,
}

impl ContextBuilder {
    pub fn chain(mut self, id: String, denom: String) -> Self {
        self.chain = Some(Chain { id, denom });
        self
    }

    pub fn rpc(mut self, url: Url, rate_limit: Option<u64>) -> Self {
        self.rpc = Some(Endpoint { url, rate_limit });
        self
    }

    pub fn height(mut self, height: Option<u64>) -> Self {
        self.height = height;
        self
    }

    pub fn soulbound_address(mut self, soulbound_address: String) -> Self {
        self.soulbound_address = Some(soulbound_address);
        self
    }

    pub fn archid_address(mut self, archid_address: String) -> Self {
        self.archid_address = Some(archid_address);
        self
    }

    pub fn liquid_finance_address(mut self, liquid_finance_address: String) -> Self {
        self.liquid_finance_address = Some(liquid_finance_address);
        self
    }

    pub fn output(mut self, output: PathBuf) -> Self {
        self.output = Some(output);
        self
    }

    pub async fn build(self) -> Result<Context> {
        let chain = self.chain.ok_or(anyhow!("missing network in config"))?;
        let rpc = self.rpc.ok_or(anyhow!("missing rpc in config"))?;
        let soulbound_address = self
            .soulbound_address
            .ok_or(anyhow!("missing soulbound address"))?;
        let archid_address = self
            .archid_address
            .ok_or(anyhow!("missing archid address"))?;
        let liquid_finance_address = self
            .liquid_finance_address
            .ok_or(anyhow!("missing liquid finance address"))?;
        let output = self.output.ok_or(anyhow!("missing output directory"))?;

        let cosmos = CosmosClient::builder()
            .url(rpc.url.clone().try_into()?)
            .rate_limit(rpc.rate_limit)
            .height(self.height)
            .build()
            .await?;

        let ctx = Context {
            chain,
            soulbound_address,
            archid_address,
            liquid_finance_address,
            cosmos: Arc::new(cosmos),
            output,
        };

        Ok(ctx)
    }
}
