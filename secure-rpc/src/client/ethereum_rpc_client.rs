use std::sync::Arc;

use json_rpc::RpcClient;

use crate::error::Error;

pub struct EthereumRpcClient(Arc<RpcClient>);

unsafe impl Send for EthereumRpcClient {}

unsafe impl Sync for EthereumRpcClient {}

impl Clone for EthereumRpcClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl EthereumRpcClient {
    pub fn new(ethereum_rpc_url: impl AsRef<str>) -> Result<Self, Error> {
        let client = RpcClient::new(ethereum_rpc_url)?;

        Ok(Self(Arc::new(client)))
    }

    pub fn rpc_client(&self) -> Arc<RpcClient> {
        self.0.clone()
    }
}
