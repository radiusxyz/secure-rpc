use std::sync::Arc;

use radius_sdk::json_rpc::client::RpcClient;

use crate::{
    client::distributed_key_generation::DistributedKeyGenerationClient, types::config::Config,
};

pub type SkdeParams = skde::delay_encryption::SkdeParams;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Clone)]
struct AppStateInner {
    config: Config,
    rpc_client: Arc<RpcClient>,
    skde_params: SkdeParams,
    distributed_key_generation_client: Option<DistributedKeyGenerationClient>,
}

impl AppState {
    pub fn new(
        config: Config,
        skde_params: SkdeParams,
        distributed_key_generation_client: Option<DistributedKeyGenerationClient>,
    ) -> Result<Self, radius_sdk::json_rpc::client::RpcClientError> {
        let rpc_client = Arc::new(RpcClient::new()?);

        let inner = AppStateInner {
            config,
            rpc_client,
            skde_params,
            distributed_key_generation_client,
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.inner.rpc_client
    }

    pub fn skde_params(&self) -> &SkdeParams {
        &self.inner.skde_params
    }

    pub fn distributed_key_generation_client(&self) -> Option<&DistributedKeyGenerationClient> {
        self.inner.distributed_key_generation_client.as_ref()
    }
}
