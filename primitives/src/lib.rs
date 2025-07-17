mod traits;
pub use traits::RpcT;

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

pub type EncryptedTxFor<C> = <<C as Context>::SecureRPCService as SecureRPCService>::EncryptedTx;

#[async_trait]
pub trait Context: RpcT {
    type SecureRPCService: SecureRPCService;
    type ExternalRpcService: ExternalRpcInterface;

    /// Get the rollup ID from the context
    fn rollup_id(&self) -> &str;

    /// Check if the encryption mode is enabled
    fn is_encrypt_enabled(&self) -> bool;

    /// Get the secure RPC service
    fn secure_rpc_service(&self) -> &Self::SecureRPCService;

    /// Get the external RPC service
    fn external_rpc_service(&self) -> &Self::ExternalRpcService;
}

#[async_trait]
/// Secure RPC service interface
pub trait SecureRPCService: RpcT {

    /// Type of the encrypted transaction
    type EncryptedTx: Serialize + DeserializeOwned + RpcT;

    /// Error type for the secure RPC service
    type Error: std::error::Error + Send + 'static;

    /// Encrypt a raw transaction with a given enc_key at a given session_id
    async fn encrypt_tx(&self, session_id: u64, raw_tx: &str, enc_key: &str) -> Result<Self::EncryptedTx, Self::Error>;
}

#[async_trait]
/// RPC interface for external resources
pub trait ExternalRpcInterface: RpcT {

    /// Type of the error for the external RPC interface
    type Error: std::error::Error + Send + 'static;
    
    /// Get the encryption key
    async fn get_enc_key(&self) -> Result<(String, u64), Self::Error>;
    
    /// Forward a RPC request to the external service
    async fn forward_rpc_request<P: Serialize + RpcT>(&self, method: &str, params: P) -> Result<serde_json::Value, Self::Error>;

    /// Forward a transaction to the external service
    async fn forward_tx<T: Serialize + RpcT>(&self, rollup_id: &str, is_encrypted: bool, tx: T) -> Result<serde_json::Value, Self::Error>;
}