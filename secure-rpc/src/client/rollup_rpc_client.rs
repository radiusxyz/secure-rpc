use std::sync::Arc;

use json_rpc::ArrayRpcClient;

use crate::error::Error;

pub struct RollupRpcClient(Arc<ArrayRpcClient>);

unsafe impl Send for RollupRpcClient {}

unsafe impl Sync for RollupRpcClient {}

impl Clone for RollupRpcClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl RollupRpcClient {
    pub fn new(ethereum_rpc_url: impl AsRef<str>) -> Result<Self, Error> {
        let client = ArrayRpcClient::new(ethereum_rpc_url)?;

        Ok(Self(Arc::new(client)))
    }

    pub fn rpc_client(&self) -> Arc<ArrayRpcClient> {
        self.0.clone()
    }
}
