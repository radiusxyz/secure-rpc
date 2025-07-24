use secure_rpc_node_primitive::{RpcEvent, OperatorEvent};
use secure_rpc_primitives::{Context, TrustedSetupFor, EncryptedTxFor};
use tokio::{sync::mpsc, task::JoinHandle};
use futures::{future::FutureExt, select_biased};

pub async fn start_secure_rpc_worker<C: Context>(_context: &C, operator_event_rx: mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>) -> (JoinHandle<()>, mpsc::Sender<RpcEvent<EncryptedTxFor<C>>>) {
    let (mut worker, tx) = SecureRpcWorker::<C>::new(operator_event_rx);
    let handle = tokio::spawn(async move {
        worker.run().await;
    });
    (handle, tx)
}

/// Main Secure RPC worker that handles the events from other workers
struct SecureRpcWorker<C: Context> {
    operator_event_rx: mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>,
    from_rpc_event_rx: mpsc::Receiver<RpcEvent<EncryptedTxFor<C>>>,
}

impl<C: Context> SecureRpcWorker<C> {
    pub fn new(operator_event_rx: mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>) -> (Self, mpsc::Sender<RpcEvent<EncryptedTxFor<C>>>) {
        let (tx, from_rpc_event_rx) = mpsc::channel(100);
        (Self { operator_event_rx, from_rpc_event_rx }, tx)
    }

    pub async fn run(&mut self) {
        loop {
            select_biased! {
                event = self.operator_event_rx.recv().fuse() => {
                    match event {
                        Some(OperatorEvent::UpdateTrustedSetup(ts)) => { /* Do Something */ }
                        Some(OperatorEvent::UpdateCommitteeList(committee_list)) => { /* Do Something */ }
                        None => { break; } // Channel is closed
                    }
                },
                event = self.from_rpc_event_rx.recv().fuse() => {
                    match event {
                        Some(RpcEvent::EncryptTx(tx)) => { /* Do Something */ }
                        None => { break; } // Channel is closed
                    }
                }
            }
        }
    }
}