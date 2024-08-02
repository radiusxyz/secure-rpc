use std::sync::Arc;

use json_rpc::RpcClient;

use crate::error::Error;

pub struct SecureRpcClient(Arc<RpcClient>);

unsafe impl Send for SecureRpcClient {}

unsafe impl Sync for SecureRpcClient {}

impl Clone for SecureRpcClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl SecureRpcClient {
    pub fn new(secure_rpc_url: impl AsRef<str>) -> Result<Self, Error> {
        let client = RpcClient::new(secure_rpc_url)?;

        Ok(Self(Arc::new(client)))
    }

    pub fn rpc_client(&self) -> Arc<RpcClient> {
        self.0.clone()
    }
}
