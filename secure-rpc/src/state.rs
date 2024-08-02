use std::sync::Arc;

use crate::{cli::Config, client::*};

pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    config: Config,
    secure_rpc_client: SecureRpcClient,
    sequencer_rpc_client: SequencerRpcClient,
    rollup_rpc_client: RollupRpcClient,
}

unsafe impl Send for AppState {}

unsafe impl Sync for AppState {}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let secure_rpc_client = SecureRpcClient::new(config.secure_rpc_url()).unwrap();
        let sequencer_rpc_client = SequencerRpcClient::new(config.sequencer_rpc_url()).unwrap();
        let rollup_rpc_client = RollupRpcClient::new(config.rollup_rpc_url()).unwrap();

        let inner = AppStateInner {
            config,
            secure_rpc_client,
            sequencer_rpc_client,
            rollup_rpc_client,
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    pub fn secure_rpc_client(&self) -> SecureRpcClient {
        self.inner.secure_rpc_client.clone()
    }

    pub fn sequencer_rpc_client(&self) -> SequencerRpcClient {
        self.inner.sequencer_rpc_client.clone()
    }

    pub fn rollup_rpc_client(&self) -> RollupRpcClient {
        self.inner.rollup_rpc_client.clone()
    }
}
