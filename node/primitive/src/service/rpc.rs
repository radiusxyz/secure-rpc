use radius_sdk::json_rpc::client::{RpcClient, Id, RpcClientError};
use serde::{Deserialize, de::DeserializeOwned};
use tokio::sync::{mpsc, oneshot};
use jsonrpsee::{core::{params::ArrayParams, traits::ToRpcParams}, rpc_params};
use async_trait::async_trait;

pub type RpcResult<T> = Result<T, RpcError>;

pub type GetEncKeyResponse = (String, u64);
pub type SendEncryptedTxResponse = serde_json::Value;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct JsonRpcResponse<T> {
    // Skip for now. Remove me if needed
    #[serde(skip)]
    jsonrpc: &'static str,
    // Skip for now. Remove me if needed
    #[serde(skip)]
    id: i64,
    result: T,
}

#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error("RPC client error: {0}")]
    RpcClientError(RpcClientError),
    #[error("Failed to serialize params")]
    SerializeError(serde_json::Error),
    #[error("Failed to deserialize response")]
    DeserializeError(serde_json::Error),
    #[error("Failed to send message to RPC worker")]
    SendError(mpsc::error::SendError<DispatchMessage>),
    #[error("Failed to receive response from RPC worker")]
    ReceiveError(oneshot::error::RecvError),
}

#[async_trait]
/// RPC interface for external resources
pub trait ExternalRpcInterface: Send + Sync + 'static {
    async fn get_enc_key(&self) -> RpcResult<GetEncKeyResponse>;

    async fn send_encrypted_tx(&self, encrypted_tx: &str) -> RpcResult<SendEncryptedTxResponse>;
}

pub enum DispatchMessage {
    /// url, method, params, response_tx
    Request(
        String,
        String,
        ArrayParams,
        oneshot::Sender<RpcResult<serde_json::Value>>
    )
}

pub struct ExternalRpcService {
    enc_key_endpoint: String,
    order_commitment_endpoint: String,
    sender: mpsc::Sender<DispatchMessage>
}

impl ExternalRpcService {
    pub fn new(tx: mpsc::Sender<DispatchMessage>, enc_key_endpoint: impl AsRef<str>, order_commitment_endpoint: impl AsRef<str>) -> Self {
        Self { 
            sender: tx,
            enc_key_endpoint: enc_key_endpoint.as_ref().to_string(),
            order_commitment_endpoint: order_commitment_endpoint.as_ref().to_string()
        }
    }

    /// General request method which returns `R` as the response type
    async fn request<R>(&self, url: &str, method: &str, params: ArrayParams) -> RpcResult<R> 
    where
        R: DeserializeOwned
    {
        let (tx, rx) = oneshot::channel();
        self.sender.send(
            DispatchMessage::Request(
                url.to_string(),
                method.to_string(),
                params,
                tx
            )
        ).await.map_err(|e| RpcError::SendError(e))?;
        let value = rx.await.map_err(|e| RpcError::ReceiveError(e))??;
        let result = serde_json::from_value(value).map_err(|e| RpcError::DeserializeError(e))?;
        Ok(result)
    }

    pub async fn get_enc_key(&self) -> RpcResult<GetEncKeyResponse> {
        self.request::<JsonRpcResponse<GetEncKeyResponse>>(&self.enc_key_endpoint, "get_enc_key", ArrayParams::new()).await.map(|r| r.result)
    }

    pub async fn send_encrypted_tx(&self, encrypted_tx: &str) -> RpcResult<SendEncryptedTxResponse> {
        self.request::<JsonRpcResponse<serde_json::Value>>(&self.order_commitment_endpoint, "send_encrypted_tx",rpc_params!(encrypted_tx)).await.map(|r| r.result)
    }
}

#[async_trait]
impl ExternalRpcInterface for ExternalRpcService {
    async fn get_enc_key(&self) -> RpcResult<GetEncKeyResponse> {
        self.get_enc_key().await
    }

    async fn send_encrypted_tx(&self, encrypted_tx: &str) -> RpcResult<SendEncryptedTxResponse> {
        self.send_encrypted_tx(encrypted_tx).await
    }
}   

pub struct RPCWorker {
    client: RpcClient,
    msg_rx: mpsc::Receiver<DispatchMessage>,
}

impl RPCWorker {
    pub fn new() -> (Self, mpsc::Sender<DispatchMessage>) {
        let client = RpcClient::new().expect("Failed to create RPC client");
        let (tx, rx) = mpsc::channel(10);
        (Self { client, msg_rx: rx }, tx)
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.msg_rx.recv().await {
            match msg {
                DispatchMessage::Request(url, method, params, response_tx) => {
                    match params.to_rpc_params() {
                        Ok(params) => {
                            let result = match self.client.request(url, method, params, Id::Null).await {
                                Ok(response) => response,
                                Err(e) => {
                                    let _ = response_tx.send(Err(RpcError::RpcClientError(e)));
                                    return;
                                }
                            };
                            let _ = response_tx.send(Ok(result));
                        },
                        Err(e) => {
                            let _ = response_tx.send(Err(RpcError::SerializeError(e)));
                        }
                    } 
                }
            }
        }
        tracing::error!("External RPC service stopped");
    }
}
