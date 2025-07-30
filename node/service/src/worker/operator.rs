use std::sync::Arc;
use secure_rpc_primitives::{Context, TrustedSetupFor, };
use secure_rpc_node_primitive::{blockchain::OperatorContract::TrustedSetupParams, service::blockchain::{BlockchainService, BlockchainEvent, BlockchainServiceError}, OperatorEvent};
use tokio::sync::mpsc;

pub async fn start_blockchain_operator_worker<C: Context>(_context: &C, blockchain_http_rpc_url: String, contract_address: String) -> anyhow::Result<(tokio::task::JoinHandle<()>, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>, TrustedSetupFor<C>, Vec<String>)> 
where
    TrustedSetupParams: Into<TrustedSetupFor<C>>,
{
    let (mut worker, rx) = BlockchainOperatorWorker::<C>::new(blockchain_http_rpc_url, contract_address);
    let (trusted_setup, committee_list) = worker.init_service().await?;
    let handle = tokio::spawn(async move { worker.run().await });
    Ok((handle, rx, trusted_setup, committee_list))
}

#[derive(Debug)]
/// Worker that handles certain state stored on-chain
pub struct BlockchainOperatorWorker<C: Context> {
    pub blockchain_service: Arc<BlockchainService>,
    pub operator_event_tx: mpsc::Sender<OperatorEvent<TrustedSetupFor<C>>>,
    pub blockchain_event_rx: mpsc::Receiver<Result<BlockchainEvent, BlockchainServiceError>>,
}

impl<C: Context> BlockchainOperatorWorker<C> 
where
    TrustedSetupParams: Into<TrustedSetupFor<C>>,
{
    pub fn new(blockchain_http_rpc_url: String, contract_address: String) -> (Self, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>) {
        let (tx, operator_event_rx) = mpsc::channel(1);
        let (blockchain_service, blockchain_event_rx) = BlockchainService::new(blockchain_http_rpc_url, contract_address);
        let blockchain_service = Arc::new(blockchain_service);

        (Self { blockchain_service, operator_event_tx: tx, blockchain_event_rx }, operator_event_rx)
    }

    pub async fn subscribe_events(&mut self) {
        loop {
            while let Some(event) = self.blockchain_event_rx.recv().await {
                match event {
                    Ok(BlockchainEvent::TrustedSetupActivated) => {
                        tracing::info!("Trusted setup activated");
                        let trusted_setup = self.blockchain_service.get_trusted_setup().await.expect("Failed to get trusted setup");
                        if let Err(e) = self.operator_event_tx.send(OperatorEvent::UpdateTrustedSetup(trusted_setup.into())).await {
                            tracing::error!("Failed to send trusted setup activated event: {:?}", e);
                        }
                    }
                    Ok(BlockchainEvent::CommitteeActivated) => {
                        tracing::info!("Committee activated");
                        let operator_list = self.blockchain_service.get_operator_rpc_urls().await.expect("Failed to get operator RPC URLs");
                        if let Err(e) = self.operator_event_tx.send(OperatorEvent::UpdateOperatorList(operator_list)).await {
                            tracing::error!("Failed to send committee activated event: {:?}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to receive blockchain event: {:?}", e);
                        continue;
                    }
                }
            }
        }
    }

    pub async fn init_service(&self) -> anyhow::Result<(TrustedSetupFor<C>, Vec<String>)> {
        let trusted_setup = self.blockchain_service.get_trusted_setup().await.expect("Failed to get trusted setup");
        let committee_list = self.blockchain_service.get_operator_rpc_urls().await.expect("Failed to get operator RPC URLs");
        Ok((trusted_setup.into(), committee_list))
    }

    /// Simple loop that runs forever
    pub async fn run(&mut self) {
        self.subscribe_events().await
    }
}