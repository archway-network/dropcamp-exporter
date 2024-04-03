use std::sync::Arc;

use cosmos_sdk_proto::cosmos::{
    base::query::v1beta1::PageRequest,
    staking::v1beta1::{QueryDelegatorDelegationsRequest, QueryDelegatorDelegationsResponse},
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
    pub async fn delegations(
        &self,
        delegator_addr: String,
    ) -> anyhow::Result<QueryDelegatorDelegationsResponse> {
        let pagination = PageRequest {
            limit: 1000,
            ..Default::default()
        };
        let request = QueryDelegatorDelegationsRequest {
            delegator_addr,
            pagination: Some(pagination),
        };

        self.rpc
            .request(
                "cosmos.staking.v1beta1.Query",
                "DelegatorDelegations",
                request,
            )
            .await
    }
}
