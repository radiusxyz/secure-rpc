mod config;
mod event;
pub use event::*;
pub use config::NodeConfig;
pub mod constants;
pub mod service;
pub use service::*;
use async_trait::async_trait;
use secure_rpc_primitives::{AsyncTask, Context, ExternalRpcInterface, SecureRpcService};
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
pub struct SecureRpcNode<S, E, AT> {
    rollup_id: String,
    is_encrypt_enabled: bool,
    secure_rpc_service: Option<S>,
    external_rpc_service: Option<E>,
    async_task: Option<AT>,
}

impl<S, E, AT> SecureRpcNode<S, E, AT> {
    /// Create a new instance of `SecureRpcNode` with secure-rpc-service `dyn SecureRpcService` and external-rpc-service `dyn ExternalRpcInterface`
    pub fn new(rollup_id: String, is_encrypt_enabled: bool) -> Self {
        Self { rollup_id, secure_rpc_service: None, external_rpc_service: None, is_encrypt_enabled, async_task: None }
    }

    pub fn with_secure_rpc_service(&mut self, secure_rpc_service: S) {
        self.secure_rpc_service = Some(secure_rpc_service);
    }

    pub fn with_external_rpc_service(&mut self, external_rpc_service: E) {
        self.external_rpc_service = Some(external_rpc_service);
    }

    pub fn with_async_task(&mut self, async_task: AT) {
        self.async_task = Some(async_task);
    }
} 

impl<S: SecureRpcService, E: ExternalRpcInterface, AT: AsyncTask> Context for SecureRpcNode<S, E, AT> {
    type SecureRpcService = S;
    type ExternalRpcService = E;
    type AsyncTask = AT;

    fn rollup_id(&self) -> &str {
        &self.rollup_id
    }

    fn is_encrypt_enabled(&self) -> bool {
        self.is_encrypt_enabled
    }

    fn secure_rpc_service(&self) -> &Self::SecureRpcService {
        self.secure_rpc_service.as_ref().expect("Secure RPC service is not initialized")
    }

    fn secure_rpc_service_mut(&mut self) -> &mut Self::SecureRpcService {
        self.secure_rpc_service.as_mut().expect("Secure RPC service is not initialized")
    }

    fn external_rpc_service(&self) -> &Self::ExternalRpcService {
        self.external_rpc_service.as_ref().expect("External RPC service is not initialized")
    }

    fn async_task(&self) -> &Self::AsyncTask {
        self.async_task.as_ref().expect("Async task is not initialized")
    }
}

#[derive(Clone)]
pub struct TaskExecutor {
    rpc_handler_event_tx: mpsc::Sender<RpcHandlerEvent>,
}

impl TaskExecutor {
    pub fn new(rpc_handler_event_tx: mpsc::Sender<RpcHandlerEvent>) -> Self {
        Self { rpc_handler_event_tx }
    }
}

#[async_trait]
impl AsyncTask for TaskExecutor {
    type Error = AsyncTaskError;

    async fn send_tx(&self, raw_tx: Vec<u8>, should_encrypt: bool) -> Result<serde_json::Value, Self::Error> {
        let (tx, rx) = oneshot::channel();
        self.rpc_handler_event_tx.send(RpcHandlerEvent::SendTx { raw_tx, should_encrypt, sender: tx }).await.map_err(|_| AsyncTaskError::SendEventError("Failed to send event".to_string()))?;
        Ok(rx.await.map_err(|_| AsyncTaskError::SendEventError("Failed to send event".to_string()))?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AsyncTaskError {
    #[error("Failed to send event: {0}")]
    SendEventError(String),
}