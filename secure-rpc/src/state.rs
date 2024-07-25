use std::sync::Arc;

use crate::{
    client::{EthereumRpcClient, SecureRpcClient},
    config::Config,
};

pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    config: Config,
    secure_rpc_client: SecureRpcClient,
    ethereum_rpc_client: EthereumRpcClient,
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
        let ethereum_rpc_client = EthereumRpcClient::new(config.ethereum_rpc_url()).unwrap();

        let inner = AppStateInner {
            config,
            secure_rpc_client,
            ethereum_rpc_client,
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

    pub fn ethereum_rpc_client(&self) -> EthereumRpcClient {
        self.inner.ethereum_rpc_client.clone()
    }
}
