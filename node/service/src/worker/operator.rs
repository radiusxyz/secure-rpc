use secure_rpc_primitives::{Context, RpcT, TrustedSetupFor, OperatorService, OperatorEvent};
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use tokio::sync::mpsc;

pub async fn start_operator_worker<C: Context, OS: OperatorService>(_context: &C, operator_service: OS) -> anyhow::Result<(tokio::task::JoinHandle<()>, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>, TrustedSetupFor<C>, Vec<String>)> 
where
    OS: RpcT,
    OS::TrustedSetup: Into<TrustedSetupFor<C>>,
{
    let (worker, rx) = OperatorWorker::<C, _>::new(operator_service);
    let (trusted_setup, committee_list) = worker.init_service().await?;
    let handle = tokio::spawn(async move { worker.run().await });
    Ok((handle, rx, trusted_setup, committee_list))
}

#[derive(Debug, Clone)]
/// Worker that handles certain state stored on-chain
pub struct OperatorWorker<C: Context, OS> {
    service: OS,
    operator_event_tx: mpsc::Sender<OperatorEvent<TrustedSetupFor<C>>>,
}

impl<C: Context, OS: OperatorService> OperatorWorker<C, OS> 
where
    OS::TrustedSetup: Into<TrustedSetupFor<C>>,
{
    pub fn new(operator_service: OS) -> (Self, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>) {
        let (tx, operator_event_rx) = mpsc::channel(1);
        (Self { service: operator_service, operator_event_tx: tx }, operator_event_rx)
    }

    pub async fn init_service(&self) -> anyhow::Result<(TrustedSetupFor<C>, Vec<String>)> {
        let trusted_setup = self.service.get_trusted_setup().await.expect("Failed to get trusted setup");
        let committee_list = self.service.get_operator_rpc_urls().await.expect("Failed to get operator RPC URLs");
        Ok((trusted_setup.into(), committee_list))
    }

    /// Subscribe to events from the blockchain
    pub async fn subscribe_events(&self) {}

    /// Simple loop that runs forever
    pub async fn run(&self) {
        loop {
        }
    }
}