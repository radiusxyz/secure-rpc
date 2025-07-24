use radius_sdk::json_rpc::client::{RpcClient, Id, RpcClientError};
use serde::{Deserialize, de::DeserializeOwned, Serialize};
use tokio::sync::{mpsc, oneshot};
use jsonrpsee::{core::{params::ArrayParams, traits::ToRpcParams}, rpc_params};
use async_trait::async_trait;
use rand::{rngs::StdRng, SeedableRng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use secure_rpc_primitives::{ExternalRpcInterface, RpcT};

pub type RpcResult<T> = Result<T, RpcError>;

pub type GetEncKeyResponse = (String, u64);
pub type SendEncryptedTxResponse = serde_json::Value;
pub type GetTxOrdererInfoResponse = Vec<(String, String)>;

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
    #[error("Empty tx orderer RPC list")]
    InternalError(String),
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

#[derive(Clone)]
pub struct ExternalRpcService {
    dkg_rpc_urls: Vec<String>,
    rollup_rpc_url: String,
    tx_orderer_rpc_url: String,
    sender: mpsc::Sender<DispatchMessage>
}

impl ExternalRpcService {
    pub fn new(tx: mpsc::Sender<DispatchMessage>, dkg_rpc_urls: Vec<String>, rollup_rpc_url: impl AsRef<str>, tx_orderer_rpc_url: impl AsRef<str>) -> Self {
        Self { 
            sender: tx,
            dkg_rpc_urls,
            rollup_rpc_url: rollup_rpc_url.as_ref().to_string(),
            tx_orderer_rpc_url: tx_orderer_rpc_url.as_ref().to_string()
        }
    }

    fn set_dkg_rpc_urls(&mut self, dkg_rpc_urls: Vec<String>) {
        self.dkg_rpc_urls = dkg_rpc_urls;
    }

    fn random_index(&self, size: usize) -> RpcResult<usize> {
        if size == 0 { return Err(RpcError::InternalError("Empty RPC list".to_string())) }
        if size == 1 { return Ok(0); }
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        Ok(rng.gen_range(0..size))
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

    pub async fn do_get_enc_key(&self) -> RpcResult<GetEncKeyResponse> {
        let index = self.random_index(self.dkg_rpc_urls.len())?;
        let url = self.dkg_rpc_urls[index].clone();
        self.request::<JsonRpcResponse<GetEncKeyResponse>>(&url, "get_enc_key", ArrayParams::new()).await.map(|r| r.result)
    }

    pub async fn do_forward_rpc_request<P: Serialize>(&self, method: &str, params: P) -> RpcResult<serde_json::Value> {
        self.request::<JsonRpcResponse<serde_json::Value>>(&self.rollup_rpc_url, method, rpc_params!(params)).await.map(|r| r.result)
    }

    pub async fn do_forward_tx<T: Serialize>(&self, rollup_id: &str, raw_tx: T, is_encrypted: bool) -> RpcResult<SendEncryptedTxResponse> {
        let method = if is_encrypted { "send_encrypted_transaction" } else { "send_raw_transaction" };
        let tx_orderer_rpc_url = self.tx_orderer_rpc_url.clone();   
        self.request::<JsonRpcResponse<serde_json::Value>>(&tx_orderer_rpc_url, method, rpc_params!(rollup_id, raw_tx)).await.map(|r| r.result)
    }
}

#[async_trait]
impl ExternalRpcInterface for ExternalRpcService {
    type Error = RpcError;

    async fn get_enc_key(&self) -> RpcResult<GetEncKeyResponse> {
        self.do_get_enc_key().await
    }

    async fn forward_rpc_request<P: Serialize + RpcT>(&self, method: &str, params: P) -> RpcResult<serde_json::Value> {
        self.do_forward_rpc_request(method, params).await
    }

    async fn forward_tx<T: Serialize + RpcT>(&self, rollup_id: &str, is_encrypted: bool, tx: T) -> RpcResult<SendEncryptedTxResponse> {
        self.do_forward_tx(rollup_id, tx, is_encrypted).await
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
