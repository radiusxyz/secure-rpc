mod config;
pub use config::NodeConfig;
mod constants;
pub mod service;
pub use service::{SkdeSecureRPCService, BlockchainService};

use secure_rpc_primitives::{Context, SecureRPCService, ExternalRpcInterface};

#[derive(Clone)]
pub struct SecureRPCNode<S, E> {
    is_encrypt_enabled: bool,
    secure_rpc_service: S,
    external_rpc_service: E
}

impl<S, E> SecureRPCNode<S, E> {
    pub fn new(secure_rpc_service: S, external_rpc_service: E, is_encrypt_enabled: bool) -> Self {
        Self { secure_rpc_service, external_rpc_service, is_encrypt_enabled }
    }
} 

impl<S: SecureRPCService, E: ExternalRpcInterface> Context for SecureRPCNode<S, E> {
    type SecureRPCService = S;
    type ExternalRpcService = E;

    fn is_encrypt_enabled(&self) -> bool {
        self.is_encrypt_enabled
    }

    fn secure_rpc_service(&self) -> &Self::SecureRPCService {
        &self.secure_rpc_service
    }

    fn external_rpc_service(&self) -> &Self::ExternalRpcService {
        &self.external_rpc_service
    }
}