use secure_rpc_primitives::{Context, TrustedSetupFor};
use secure_rpc_node_primitive::{blockchain::OperatorContract::TrustedSetupParams, service::blockchain::BlockchainService, OperatorEvent};
use tokio::sync::mpsc;
use futures_util::{select_biased, StreamExt, future::FutureExt}; 

pub async fn start_blockchain_operator_worker<C: Context>(_context: &C, blockchain_http_rpc_url: String, contract_address: String) -> anyhow::Result<(tokio::task::JoinHandle<()>, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>, TrustedSetupFor<C>, Vec<String>)> 
where
    TrustedSetupParams: Into<TrustedSetupFor<C>>,
{
    let (worker, rx) = BlockchainOperatorWorker::<C>::new(blockchain_http_rpc_url, contract_address);
    let (trusted_setup, committee_list) = worker.init_service().await?;
    let handle = tokio::spawn(async move { worker.run().await });
    Ok((handle, rx, trusted_setup, committee_list))
}

#[derive(Debug, Clone)]
/// Worker that handles certain state stored on-chain
pub struct BlockchainOperatorWorker<C: Context> {
    pub blockchain_service: BlockchainService,
    operator_event_tx: mpsc::Sender<OperatorEvent<TrustedSetupFor<C>>>,
}

impl<C: Context> BlockchainOperatorWorker<C> 
where
    TrustedSetupParams: Into<TrustedSetupFor<C>>,
{
    pub fn new(blockchain_http_rpc_url: String, contract_address: String) -> (Self, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>) {
        let (tx, operator_event_rx) = mpsc::channel(1);
        let blockchain_service = BlockchainService::new(blockchain_http_rpc_url, contract_address);

        (Self { blockchain_service, operator_event_tx: tx }, operator_event_rx)
    }

    pub async fn subscribe_events(&self) {
        let mut trusted_setup_activated_stream = self.blockchain_service.contract_instance.TrustedSetupActivated_filter().watch().await.expect("Failed to subscribe to trusted setup activated events").into_stream();
        let mut committee_activated_stream = self.blockchain_service.contract_instance.CommitteeActivated_filter().watch().await.expect("Failed to subscribe to committee activated events").into_stream();

        loop {
            select_biased! {
                maybe_res = trusted_setup_activated_stream.next().fuse() => {
                    match maybe_res {
                        Some(res) => {
                            if let Ok(event) = res {
                                tracing::debug!("Trusted setup activated at block: {:?}", event.0.activationBlock);
                                if let Ok(trusted_setup) = self.blockchain_service.get_trusted_setup().await {
                                    self.operator_event_tx.send(OperatorEvent::UpdateTrustedSetup(trusted_setup.into())).await.expect("Failed to send operator event");
                                }
                            } else {
                                // TODO: Handle this error better
                                tracing::error!("Failed to decode trusted setup activated event: {:?}", res);
                                continue;
                            }
                        }
                        _ => continue,
                    }
                },
                maybe_res = committee_activated_stream.next().fuse() => {
                    match maybe_res {
                        Some(res) => {
                            if let Ok(event) = res {
                                tracing::debug!("Committee activated at block: {:?}", event.0.activationBlock);
                                if let Ok(operator_list) = self.blockchain_service.get_operator_rpc_urls().await {
                                    self.operator_event_tx.send(OperatorEvent::UpdateOperatorList(operator_list)).await.expect("Failed to send operator event");
                                }
                            } else {
                                // TODO: Handle this error better
                                tracing::error!("Failed to decode committee activated event: {:?}", res);
                                continue;
                            }
                        }
                        _ => continue,
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
    pub async fn run(&self) {
        self.subscribe_events().await
    }
}