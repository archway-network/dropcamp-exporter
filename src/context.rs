use std::{path::PathBuf, sync::Arc};

use anyhow::*;
use url::Url;

use crate::{
    clients::{AstrovaultClient, CosmosClient},
    config::Ranking,
    csv,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Endpoint {
    pub url: Url,
    pub req_second: Option<u64>,
    pub api_key: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub soulbound_address: String,
    pub archid_address: String,
    pub liquid_finance_address: String,
    pub cosmos: Arc<CosmosClient>,
    pub astrovault: Arc<AstrovaultClient>,
    pub ranking: Ranking,
    output: PathBuf,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::default()
    }

    pub fn create_output_folder(&self) -> Result<()> {
        std::fs::create_dir_all(&self.output)?;
        Ok(())
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
    rpc: Option<Endpoint>,
    height: Option<u64>,
    soulbound_address: Option<String>,
    archid_address: Option<String>,
    liquid_finance_address: Option<String>,
    astrovault: Option<Endpoint>,
    ranking_path: Option<PathBuf>,
    output: Option<PathBuf>,
}

impl ContextBuilder {
    pub fn rpc(mut self, url: Url, req_second: Option<u64>) -> Self {
        self.rpc = Some(Endpoint {
            url,
            req_second,
            api_key: None,
        });
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

    pub fn astrovault(
        mut self,
        url: Url,
        req_second: Option<u64>,
        api_key: Option<String>,
    ) -> Self {
        self.astrovault = Some(Endpoint {
            url,
            req_second,
            api_key,
        });
        self
    }

    pub fn ranking_path(mut self, ranking_path: PathBuf) -> Self {
        self.ranking_path = Some(ranking_path);
        self
    }

    pub fn output(mut self, output: PathBuf) -> Self {
        self.output = Some(output);
        self
    }

    pub async fn build(self) -> Result<Context> {
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

        let rpc = self.rpc.ok_or(anyhow!("missing rpc arguments"))?;
        let cosmos = CosmosClient::new(rpc.url, rpc.req_second, self.height).await?;

        let av_endpoint = self
            .astrovault
            .clone()
            .ok_or(anyhow!("missing astrovault arguments"))?;
        let astrovault = AstrovaultClient::builder(av_endpoint.url.clone())
            .req_second(av_endpoint.req_second)
            .api_key(av_endpoint.api_key.clone())
            .build()
            .await?;

        let ranking_path = self
            .ranking_path
            .ok_or(anyhow!("missing ranking config file path"))?;
        let ranking = Ranking::load(ranking_path)?;

        let ctx = Context {
            soulbound_address,
            archid_address,
            liquid_finance_address,
            cosmos: Arc::new(cosmos),
            astrovault: Arc::new(astrovault),
            ranking,
            output,
        };

        Ok(ctx)
    }
}
