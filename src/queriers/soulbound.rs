use futures::stream::{self, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Extension {
    pub id: String,
    pub description: String,
    pub social_score: u16,
}

#[derive(Clone, Debug)]
pub struct TokenInfo {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub social_score: u16,
}

pub struct SoulboundToken {
    ctx: Arc<Context>,
}

impl SoulboundToken {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx }
    }

    pub async fn all_tokens(&self) -> Result<Vec<TokenInfo>> {
        tracing::info!(%self.ctx.soulbound_address, "querying soulbound token owners");

        let mut all_owners: Vec<TokenInfo> = Vec::new();
        let mut start_after: Option<String> = None;
        let limit = 100;

        loop {
            tracing::info!(start_after, "fetching the next {} soulbound tokens", limit);

            let query = cw721::Cw721QueryMsg::AllTokens {
                start_after,
                limit: Some(limit),
            };
            let response: cw721::TokensResponse = self.query_contract(&query).await?;
            let count = response.tokens.len();
            tracing::info!(%count, "found soulbound tokens");

            start_after = response.tokens.last().cloned();

            let owners: Vec<TokenInfo> = stream::iter(response.tokens)
                .map(|token_id| self.token_info(token_id))
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

    async fn token_info(&self, token_id: String) -> Result<TokenInfo> {
        tracing::debug!(%token_id, "querying soulbound token owner");

        let query = cw721::Cw721QueryMsg::AllNftInfo {
            token_id: token_id.clone(),
            include_expired: Some(true),
        };

        let response: cw721::AllNftInfoResponse<Extension> = self.query_contract(&query).await?;
        let token = TokenInfo {
            id: token_id,
            name: response.info.extension.id,
            owner: response.access.owner,
            social_score: response.info.extension.social_score,
        };

        tracing::debug!(
            id = token.id,
            name = token.name,
            owner = token.owner,
            social_score = token.social_score,
            "found soulbound token"
        );

        Ok(token)
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
