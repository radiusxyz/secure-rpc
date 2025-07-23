mod config;
mod event;
pub use event::*;
pub use config::NodeConfig;
pub mod constants;
pub mod service;
pub use service::*;

use secure_rpc_primitives::{Context, SecureRpcService, ExternalRpcInterface};

#[derive(Clone)]
pub struct SecureRpcNode<S, E> {
    rollup_id: String,
    is_encrypt_enabled: bool,
    secure_rpc_service: Option<S>,
    external_rpc_service: Option<E>,
}

impl<S, E> SecureRpcNode<S, E> {
    /// Create a new instance of `SecureRpcNode` with secure-rpc-service `dyn SecureRpcService` and external-rpc-service `dyn ExternalRpcInterface`
    pub fn new(rollup_id: String, is_encrypt_enabled: bool) -> Self {
        Self { rollup_id, secure_rpc_service: None, external_rpc_service: None, is_encrypt_enabled }
    }
} 

impl<S: SecureRpcService, E: ExternalRpcInterface> Context for SecureRpcNode<S, E> {
    type SecureRpcService = S;
    type ExternalRpcService = E;

    fn rollup_id(&self) -> &str {
        &self.rollup_id
    }

    fn is_encrypt_enabled(&self) -> bool {
        self.is_encrypt_enabled
    }

    fn secure_rpc_service(&self) -> &Self::SecureRpcService {
        self.secure_rpc_service.as_ref().expect("Secure RPC service is not initialized")
    }

    fn external_rpc_service(&self) -> &Self::ExternalRpcService {
        self.external_rpc_service.as_ref().expect("External RPC service is not initialized")
    }
}