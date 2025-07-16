use jsonrpsee::{core::{params::ArrayParams, traits::ToRpcParams}, rpc_params};
use tokio::sync::{mpsc, oneshot};
use secure_rpc_node_primitive::service::rpc::{DispatchMessage, ExternalRpcService, RpcError};
use radius_sdk::json_rpc::server::RpcServer;

pub async fn start_external_rpc_worker(enc_key_endpoint: impl AsRef<str>, order_commitment_endpoint: impl AsRef<str>) -> (ExternalRpcService, tokio::task::JoinHandle<()>) {
    let (rpc_worker, tx) = ExternalRPCWorker::new();
    let handle = tokio::spawn(async move {
        rpc_worker.run().await;
    });
    (ExternalRpcService::new(tx, enc_key_endpoint, order_commitment_endpoint), handle)
}

pub struct ExternalRPCWorker {
    client: RpcClient,
    msg_rx: mpsc::Receiver<DispatchMessage>,
}

impl ExternalRPCWorker {
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