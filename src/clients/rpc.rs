use cosmos_sdk_proto::prost::Message;
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};

/// A RPC client wrapper that queries ProtoBuf
/// encoded message for a single block height.
#[derive(Debug)]
pub struct RpcClient {
    inner: HttpClient,
    height: Height,
}

impl RpcClient {
    pub fn new(inner: HttpClient, height: Height) -> Self {
        Self { inner, height }
    }

    #[tracing::instrument(fields(height = self.height.value()), skip(self, data))]
    pub async fn request<T, R>(&self, service: &str, method: &str, data: T) -> anyhow::Result<R>
    where
        T: Message + Default,
        R: Message + Default,
    {
        tracing::debug!(?data, "request");

        let path = format!("/{service}/{method}");
        let response = self
            .inner
            .abci_query(Some(path), data.encode_to_vec(), Some(self.height), false)
            .await?;

        let response: R = R::decode(&*response.value)?;
        tracing::debug!(?response, "response");

        Ok(response)
    }
}
