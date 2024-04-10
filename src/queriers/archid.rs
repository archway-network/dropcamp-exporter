use serde::{Deserialize, Serialize};

use crate::prelude::*;

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
    ctx: Arc<Context>,
}

impl ArchIdRegistry {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx }
    }

    #[tracing::instrument(skip(self))]
    pub async fn resolve_domains(&self, address: String) -> Result<Vec<String>> {
        tracing::debug!(%self.ctx.archid_address, "resolving ArchID names for address");

        let query = QueryMsg::ResolveAddress { address };
        let response: ResolveAddressResponse = self
            .ctx
            .query_contract(self.ctx.archid_address.clone(), &query)
            .await?;
        let names = response.names.unwrap_or_default();
        tracing::debug!(count = names.len(), "found ArchID names");

        Ok(names)
    }
}
