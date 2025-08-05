use secure_rpc_primitives::{Context, TrustedSetupFor, };
use secure_rpc_node_primitive::{service::operator::ssv::{DkgBApp, BAppService, BAppServiceError, BAppEvent}, OperatorEvent};
use tokio::sync::mpsc;

pub async fn start_bapp_operator_worker<C: Context>(_context: &C, blockchain_http_rpc_url: String, blockchain_ws_rpc_url: String, contract_address: String) -> anyhow::Result<(tokio::task::JoinHandle<()>, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>, TrustedSetupFor<C>, Vec<String>)> 
where
    DkgBApp::TrustedSetupParams: Into<TrustedSetupFor<C>>,
{
    let (mut worker, rx) = BAppSubscriber::<C>::new(blockchain_http_rpc_url, contract_address);
    let (trusted_setup, committee_list) = worker.init_service().await?;
    let handle = tokio::spawn(async move { worker.run(blockchain_ws_rpc_url).await });
    Ok((handle, rx, trusted_setup, committee_list))
}

#[derive(Debug)]
/// Worker that handles certain state stored on-chain
pub struct BAppSubscriber<C: Context> {
    pub bapp_service: BAppService,
    pub operator_event_tx: mpsc::Sender<OperatorEvent<TrustedSetupFor<C>>>,
    pub bapp_event_rx: mpsc::Receiver<Result<BAppEvent, BAppServiceError>>,
}

impl<C: Context> BAppSubscriber<C> 
where
    DkgBApp::TrustedSetupParams: Into<TrustedSetupFor<C>>,
{
    pub fn new(blockchain_http_rpc_url: String, contract_address: String) -> (Self, mpsc::Receiver<OperatorEvent<TrustedSetupFor<C>>>) {
        let (bapp_service, bapp_event_rx) = BAppService::new(blockchain_http_rpc_url, contract_address);
        let (tx, operator_event_rx) = mpsc::channel(1);
        (Self { bapp_service, operator_event_tx: tx, bapp_event_rx }, operator_event_rx)
    }

    pub async fn init_service(&self) -> anyhow::Result<(TrustedSetupFor<C>, Vec<String>)> {
        let trusted_setup = self.bapp_service.get_active_trusted_setup().await.expect("Failed to get trusted setup");
        let committee_list = self.bapp_service.get_operator_rpc_urls().await.expect("Failed to get operator RPC URLs");
        Ok((trusted_setup.into(), committee_list))
    }

    pub async fn send_updated_operator_list(&self) {
        if let Ok(operator_rpc_urls) = self.bapp_service.get_operator_rpc_urls().await {
            tracing::info!("Operator RPC URLs: {:?}", operator_rpc_urls);
            if let Err(e) = self.operator_event_tx.send(OperatorEvent::UpdateOperatorList(operator_rpc_urls)).await {
                tracing::error!("Failed to send operator RPC URLs: {:?}", e);
            }
        } else {
            tracing::error!("Failed to get operator RPC URLs");
        }
    }

    pub async fn send_updated_trusted_setup(&self) {
        if let Ok(trusted_setup) = self.bapp_service.get_active_trusted_setup().await {
            tracing::info!("Updating trusted setup");
            if let Err(e) = self.operator_event_tx.send(OperatorEvent::UpdateTrustedSetup(trusted_setup.into())).await {
                tracing::error!("Failed to send trusted setup: {:?}", e);
            }
        } else {
            tracing::error!("Failed to get trusted setup");
        }
    }

    pub async fn handle_task_created(&self) {}

    pub async fn handle_response_task(&self) {}

    pub async fn handle_task_completed(&self) {
        self.send_updated_operator_list().await;
        self.send_updated_trusted_setup().await;
    }

    pub async fn subscribe_events(&mut self, blockchain_ws_rpc_url: String) {
        self.bapp_service.subscribe_events(blockchain_ws_rpc_url).await;
        while let Some(event) = self.bapp_event_rx.recv().await {
            tracing::info!("Received event: {:?}", event);
            match event {
                Ok(BAppEvent::TaskCreated(_)) => {
                    self.handle_task_created().await;
                }
                Ok(BAppEvent::TaskResponse(_)) => {
                    self.handle_response_task().await;
                }
                Ok(BAppEvent::TaskCompleted(_)) => {
                    self.handle_task_completed().await;
                }
                Err(e) => {
                    tracing::error!("Failed to receive event: {:?}", e);
                }
            }
        }
    }

    pub async fn run(&mut self, blockchain_ws_rpc_url: String) {
        self.subscribe_events(blockchain_ws_rpc_url).await;
    }
}