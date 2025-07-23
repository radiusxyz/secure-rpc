/// Type of events that are sent from the RPC worker to the secure RPC worker
pub enum RpcEvent<Tx> {
    EncryptTx(Tx),
}

/// Type of events that are sent from the operator worker to the secure RPC worker
pub enum OperatorEvent<TS> {
    UpdateTrustedSetup(TS),
    UpdateCommitteeList(Vec<String>),
}