use std::sync::Arc;

use json_rpc::EthRpcClient;

use crate::error::Error;

pub struct RollupRpcClient(Arc<EthRpcClient>);

unsafe impl Send for RollupRpcClient {}

unsafe impl Sync for RollupRpcClient {}

impl Clone for RollupRpcClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl RollupRpcClient {
    pub fn new(ethereum_rpc_url: impl AsRef<str>) -> Result<Self, Error> {
        let client = EthRpcClient::new(ethereum_rpc_url)?;

        Ok(Self(Arc::new(client)))
    }

    pub fn rpc_client(&self) -> Arc<EthRpcClient> {
        self.0.clone()
    }
}
