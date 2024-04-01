use serde::{de::DeserializeOwned, Deserialize, Serialize};

use anyhow::*;

use crate::context::Context;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum QueryMsg {
    ResolveAddress { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct ResolveAddressResponse {
    pub names: Option<Vec<String>>,
}

pub struct ArchIdRegistry {
    ctx: Context,
}

impl ArchIdRegistry {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    #[tracing::instrument(skip(self))]
    pub async fn resolve_domains(&self, address: String) -> Result<Vec<String>> {
        tracing::debug!(%self.ctx.archid_address, "resolving ArchID names for address");

        let query = QueryMsg::ResolveAddress { address };
        let response: ResolveAddressResponse = self.query_contract(&query).await?;
        let names = response.names.unwrap_or_default();
        tracing::debug!(count = names.len(), "found ArchID names");

        Ok(names)
    }

    async fn query_contract<T, R>(&self, data: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        self.ctx
            .cosmos
            .cosmwasm
            .smart_contract_state(self.ctx.archid_address.clone(), data)
            .await
    }
}
