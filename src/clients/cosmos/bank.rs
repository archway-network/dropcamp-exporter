use std::sync::Arc;

use cosmos_sdk_proto::cosmos::bank::v1beta1::{
    QueryBalanceRequest, QueryBalanceResponse, QuerySupplyOfRequest, QuerySupplyOfResponse,
};

use super::super::rpc::RpcClient;

#[derive(Debug, Clone)]
pub struct QueryClient {
    rpc: Arc<RpcClient>,
}

impl QueryClient {
    pub fn new(rpc: Arc<RpcClient>) -> Self {
        Self { rpc }
    }

    #[tracing::instrument(skip(self))]
    pub async fn total_supply(&self, denom: String) -> anyhow::Result<QuerySupplyOfResponse> {
        let request = QuerySupplyOfRequest { denom };

        self.rpc
            .request("cosmos.bank.v1beta1.Query", "SupplyOf", request)
            .await
    }

    #[tracing::instrument(skip(self))]
    pub async fn balance(
        &self,
        address: String,
        denom: String,
    ) -> anyhow::Result<QueryBalanceResponse> {
        let request = QueryBalanceRequest { address, denom };

        self.rpc
            .request("cosmos.bank.v1beta1.Query", "Balance", request)
            .await
    }
}
