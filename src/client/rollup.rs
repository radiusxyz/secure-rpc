use std::sync::Arc;

use radius_sdk::json_rpc::client::{RpcClient, RpcClientError};

pub struct RollupRpcClient {
    inner: Arc<RollupRpcClientInner>,
}

struct RollupRpcClientInner {
    rollup_rpc_url: String,
    rpc_client: RpcClient,
}

impl Clone for RollupRpcClient {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl RollupRpcClient {
    pub fn new(rollup_rpc_url: impl AsRef<str>) -> Result<Self, RpcClientError> {
        let rollup_rpc_url = rollup_rpc_url.as_ref().to_owned();
        let rpc_client = RpcClient::new()?;

        Ok(Self {
            inner: RollupRpcClientInner {
                rollup_rpc_url,
                rpc_client,
            }
            .into(),
        })
    }
}
