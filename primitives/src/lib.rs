mod traits;
pub use traits::{RpcT, ErrorT};

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};


pub type EncryptedTxFor<C> = <<C as Context>::SecureRpcService as SecureRpcService>::EncryptedTx;
pub type RawTxFor<C> = <<C as Context>::SecureRpcService as SecureRpcService>::RawTx;
pub type TrustedSetupFor<C> = <<C as Context>::SecureRpcService as SecureRpcService>::TrustedSetup;

#[async_trait]
pub trait Context: RpcT {
    type SecureRpcService: SecureRpcService;
    type ExternalRpcService: ExternalRpcInterface;
    type AsyncTask: AsyncTask;

    /// Get the rollup ID from the context
    fn rollup_id(&self) -> &str;

    /// Check if the encryption mode is enabled
    fn is_encrypt_enabled(&self) -> bool;

    /// Get the secure RPC service
    fn secure_rpc_service(&self) -> &Self::SecureRpcService;

    /// Get the secure RPC service mutably
    fn secure_rpc_service_mut(&mut self) -> &mut Self::SecureRpcService;

    /// Get the external RPC service
    fn external_rpc_service(&self) -> &Self::ExternalRpcService;

    /// Get the async task
    fn async_task(&self) -> &Self::AsyncTask;
}

#[async_trait]
/// Secure RPC service interface
pub trait SecureRpcService: RpcT {
    /// Type of the trusted setup
    type TrustedSetup: Serialize + DeserializeOwned + RpcT;

    /// Type of the raw transaction
    type RawTx: Serialize + DeserializeOwned + RpcT;

    /// Type of the encrypted transaction
    type EncryptedTx: Serialize + DeserializeOwned + RpcT;

    /// Error type for the secure RPC service
    type Error: ErrorT;

    fn update_trusted_setup(&mut self, trusted_setup: Self::TrustedSetup);

    /// Encrypt a raw transaction with a given enc_key at a given session_id
    async fn encrypt_tx(&self, session_id: u64, raw_tx: &[u8], enc_key: &str) -> Result<Self::EncryptedTx, Self::Error>;
}

#[async_trait]
/// RPC interface for external resources
pub trait ExternalRpcInterface: RpcT {

    /// Type of the error for the external RPC interface
    type Error: ErrorT;
    
    /// Get the encryption key
    async fn get_enc_key(&self, url: &str) -> Result<(String, u64), Self::Error>;
    
    /// Forward a RPC request to the external service
    async fn forward_rpc_request<P: Serialize + RpcT>(&self, method: &str, params: P) -> Result<serde_json::Value, Self::Error>;

    /// Forward a transaction to the external service
    async fn forward_tx<T: Serialize + RpcT>(&self, url: &str, rollup_id: &str, is_encrypted: bool, tx: T) -> Result<serde_json::Value, Self::Error>;
}

/// API for operator service(e.g SSV)
#[async_trait]
pub trait Operator {

    /// Type of the trusted setup this operator service is using
    type TrustedSetup;

    /// Type of the error for the operator service
    type Error: ErrorT;

    /// Get the trusted setup
    async fn get_active_trusted_setup(&self) -> Option<Self::TrustedSetup>;

    /// Get the operator's RPC URLs
    async fn get_operator_rpc_urls(&self) -> Option<Vec<String>>;
}

#[async_trait]
pub trait AsyncTask: RpcT {
    /// Type of the error for the async task
    type Error: ErrorT;
    
    /// Send a transaction to the secure RPC worker
    async fn send_tx(&self, tx: Vec<u8>, should_encrypt: bool) -> Result<serde_json::Value, Self::Error>;
}