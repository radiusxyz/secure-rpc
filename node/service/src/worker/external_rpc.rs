use tokio::sync::mpsc;
use jsonrpsee::core::traits::ToRpcParams;
use secure_rpc_node_primitive::service::rpc::{DispatchMessage, ExternalRpcService, RpcError};
use radius_sdk::json_rpc::client::{RpcClient, Id};

pub async fn start_external_rpc_worker(rollup_rpc_url: &str) -> (ExternalRpcService, tokio::task::JoinHandle<()>) {
    let (rpc_worker, tx) = ExternalRPCWorker::new();
    let handle = tokio::spawn(async move {
        rpc_worker.run().await;
    });
    (ExternalRpcService::new(rollup_rpc_url.to_string(), tx), handle)
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