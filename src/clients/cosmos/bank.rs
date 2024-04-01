use std::sync::Arc;

use cosmos_sdk_proto::cosmos::{
    bank::v1beta1::{QueryAllBalancesRequest, QueryAllBalancesResponse},
    base::query::v1beta1::PageRequest,
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
    pub async fn balances(&self, address: String) -> anyhow::Result<QueryAllBalancesResponse> {
        let pagination = PageRequest {
            limit: 1000,
            ..Default::default()
        };
        let request = QueryAllBalancesRequest {
            address,
            pagination: Some(pagination),
        };

        self.rpc
            .request("cosmos.bank.v1beta1.Query", "AllBalances", request)
            .await
    }
}
