use std::{any::type_name, sync::Arc};

use anyhow::*;
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    QuerySmartContractStateRequest, QuerySmartContractStateResponse,
};
use serde::{de::DeserializeOwned, Serialize};

use super::super::rpc::RpcClient;

#[derive(Debug, Clone)]
pub struct QueryClient {
    rpc: Arc<RpcClient>,
}

impl QueryClient {
    pub fn new(rpc: Arc<RpcClient>) -> Self {
        Self { rpc }
    }

    #[tracing::instrument(skip(self, data))]
    pub async fn smart_contract_state<T, R>(&self, address: String, data: &T) -> Result<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let query_data = serde_json::to_vec(data)
            .map_err(|e| anyhow!("failed to serialize {}: {}", type_name::<T>(), e))?;

        let request = QuerySmartContractStateRequest {
            address,
            query_data,
        };

        let response: QuerySmartContractStateResponse = self
            .rpc
            .request("cosmwasm.wasm.v1.Query", "SmartContractState", request)
            .await?;

        let data = serde_json::from_slice(response.data.as_slice())
            .map_err(|e| anyhow!("failed to parse response into {}: {}", type_name::<R>(), e))?;

        Ok(data)
    }
}
