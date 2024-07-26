use std::sync::Arc;

use json_rpc::RpcClient;

use crate::error::Error;

pub struct SequencerRpcClient(Arc<RpcClient>);

unsafe impl Send for SequencerRpcClient {}

unsafe impl Sync for SequencerRpcClient {}

impl Clone for SequencerRpcClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl SequencerRpcClient {
    pub fn new(sequencer_rpc_url: impl AsRef<str>) -> Result<Self, Error> {
        let client = RpcClient::new(sequencer_rpc_url)?;

        Ok(Self(Arc::new(client)))
    }

    pub fn rpc_client(&self) -> Arc<RpcClient> {
        self.0.clone()
    }
}
