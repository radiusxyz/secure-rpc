use tokio::sync::oneshot::Sender;

/// Type of events that are sent from the RPC worker to the secure RPC worker
#[derive(Debug)]
pub enum RpcHandlerEvent {
    SendTx { raw_tx: Vec<u8>, should_encrypt: bool, sender: Sender<serde_json::Value> },
}

/// Type of events that are sent from the operator worker to the secure RPC worker
#[derive(Debug, Clone)]
pub enum OperatorEvent<TS> {
    UpdateTrustedSetup(TS),
    UpdateOperatorList(Vec<String>),
}