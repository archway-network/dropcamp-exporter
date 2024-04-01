use std::{collections::HashSet, sync::Arc};

use futures::stream::{self, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};

use crate::prelude::*;

pub struct SoulboundToken {
    ctx: Arc<Context>,
}

impl SoulboundToken {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx }
    }

    pub async fn get_owners(&self) -> Result<HashSet<String>> {
        tracing::info!(%self.ctx.soulbound_address, "querying soulbound token owners");

        let mut all_owners: HashSet<String> = HashSet::new();
        let mut start_after: Option<String> = None;

        loop {
            tracing::info!(start_after, "fetching soulbound tokens");

            let query = cw721::Cw721QueryMsg::AllTokens {
                start_after,
                limit: Some(100),
            };
            let response: cw721::TokensResponse = self.query_contract(&query).await?;
            let count = response.tokens.len();
            tracing::info!(%count, "found soulbound tokens");

            start_after = response.tokens.last().cloned();

            let owners: HashSet<String> = stream::iter(response.tokens)
                .map(|token_id| self.get_token_owner(token_id))
                .buffer_unordered(10)
                .try_collect()
                .await?;

            all_owners.extend(owners);

            if count < 100 {
                break;
            }
        }

        tracing::info!(count = all_owners.len(), "total soulbound token owners");

        Ok(all_owners)
    }

    async fn get_token_owner(&self, token_id: String) -> Result<String> {
        tracing::debug!(%token_id, "querying soulbound token owner");

        let query = cw721::Cw721QueryMsg::OwnerOf {
            token_id,
            include_expired: Some(true),
        };

        let response: cw721::OwnerOfResponse = self.query_contract(&query).await?;
        tracing::debug!(owner = response.owner, "found soulbound token owner");

        Ok(response.owner)
    }

    async fn query_contract<T, R>(&self, data: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        self.ctx
            .cosmos
            .cosmwasm
            .smart_contract_state(self.ctx.soulbound_address.clone(), data)
            .await
    }
}
