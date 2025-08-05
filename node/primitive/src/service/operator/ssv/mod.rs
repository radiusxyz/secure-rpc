
use std::fmt::Debug;

use alloy::{
    primitives::Address as EthAddress, providers::{ProviderBuilder, RootProvider, WsConnect}, pubsub::PubSubFrontend, sol, transports::http::{reqwest::Url, Client, Http},
    rpc::types::Log,
};
use futures::Stream;
use tokio::sync::mpsc;
pub use DkgBApp::{DkgBAppInstance, TaskCreated, TaskResponse, TaskCompleted};
use futures_util::StreamExt;

sol! {
    #![sol(rpc, all_derives)]
    DkgBApp,
    "src/service/operator/ssv/contract/DkgBApp.json"
}

async fn spawn_event_handler<T, E>(
    mut event_stream: impl Stream<Item = Result<T, E>> + Unpin,
    blockchain_event_tx: mpsc::Sender<Result<BAppEvent, BAppServiceError>>
) 
where
    T: Into<BAppEvent> + Debug + Clone,
{
    while let Some(res) = event_stream.next().await {
        if let Ok(event) = res {
            if let Err(e) = blockchain_event_tx.send(Ok(event.clone().into())).await {
                tracing::error!("Failed to send {:?} event: {:?}", event, e);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BAppEvent {
    TaskCreated(TaskCreated),
    TaskResponse(TaskResponse),
    TaskCompleted(TaskCompleted),
}

macro_rules! impl_bapp_event_from {
    ($($event: ident), *) => {
        $(
            impl From<($event, Log)> for BAppEvent {
                fn from((event, _): ($event, Log)) -> Self {
                    BAppEvent::$event(event)
                }
            }
        )*
    }
}

impl_bapp_event_from!(TaskCreated, TaskResponse, TaskCompleted);


// TODO: Refactor to use generic wrapper
#[derive(Debug, Clone)]
pub struct BAppService {
    pub blockchain_event_tx: mpsc::Sender<Result<BAppEvent, BAppServiceError>>,
    pub contract_address: EthAddress,
    pub state_provider: DkgBAppInstance<Http<Client>, RootProvider<Http<Client>>>,
    pub event_pubsub: Option<DkgBAppInstance<PubSubFrontend, RootProvider<PubSubFrontend>>>,
}

impl BAppService {
    pub fn new(blockchain_http_rpc_url: String, contract_address: String) -> (Self, mpsc::Receiver<Result<BAppEvent, BAppServiceError>>) {
        let http_provider = ProviderBuilder::new().on_http(Url::parse(&blockchain_http_rpc_url).unwrap());
        let contract_address = contract_address.parse::<EthAddress>().expect("Invalid contract address");
        let (blockchain_event_tx, blockchain_event_rx) = mpsc::channel(2);
        let contract_instance = DkgBAppInstance::new(contract_address, http_provider);
        (Self { blockchain_event_tx, contract_address, state_provider: contract_instance, event_pubsub: None }, blockchain_event_rx)
    }

    async fn connect_to_ws(&mut self, blockchain_ws_rpc_url: String) {
        let ws_provider = ProviderBuilder::new().on_ws(WsConnect::new(Url::parse(&blockchain_ws_rpc_url).unwrap())).await.expect("Failed to connect to WS");
        let event_pubsub = DkgBAppInstance::new(self.contract_address, ws_provider);
        self.event_pubsub = Some(event_pubsub);
    }

    pub async fn get_active_trusted_setup(&self) -> Result<DkgBApp::TrustedSetupParams, BAppServiceError> 
    {
        let res = self.state_provider.getActiveTrustedSetup().call().await.map_err(|_| BAppServiceError::FailedToGetTrustedSetup)?;
        return Ok(res._0)
    }

    pub async fn get_operator_rpc_urls(&self) -> Result<Vec<String>, BAppServiceError> {
        let res = self.state_provider.getActiveCommitteeList().call().await.map_err(|_| BAppServiceError::FailedToGetOperatorList)?;
        let operator_rpc_urls = res._0.iter().map(|c| c.externalRpcUrl.clone()).collect();
        return Ok(operator_rpc_urls)
    }

    pub async fn subscribe_events(&mut self, blockchain_ws_rpc_url: String) {
        self.connect_to_ws(blockchain_ws_rpc_url).await;
        let task_created_event = self.event_pubsub.as_ref().unwrap().TaskCreated_filter().subscribe().await.map_err(|e| {
            tracing::error!("Failed to subscribe to task created event: {:?}", e);
            BAppServiceError::FailedToSubscribeEvents
        }).unwrap().into_stream();
        let task_response_event = self.event_pubsub.as_ref().unwrap().TaskResponse_filter().subscribe().await.map_err(|e| {
            tracing::error!("Failed to subscribe to task response event: {:?}", e);
            BAppServiceError::FailedToSubscribeEvents
        }).unwrap().into_stream();
        let task_completed_event = self.event_pubsub.as_ref().unwrap().TaskCompleted_filter().subscribe().await.map_err(|e| {
            tracing::error!("Failed to subscribe to task completed event: {:?}", e);
            BAppServiceError::FailedToSubscribeEvents
        }).unwrap().into_stream();
        let blockchain_event_tx = self.blockchain_event_tx.clone();
        tokio::spawn(spawn_event_handler(task_created_event, blockchain_event_tx.clone()));
        tokio::spawn(spawn_event_handler(task_response_event, blockchain_event_tx.clone()));
        tokio::spawn(spawn_event_handler(task_completed_event, blockchain_event_tx.clone()));
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BAppServiceError {
    #[error("Failed to get trusted setup")]
    FailedToGetTrustedSetup,
    #[error("Failed to decode trusted setup")]
    FailedToDecodeTrustedSetup,
    #[error("Failed to get operator list")]
    FailedToGetOperatorList,
    #[error("Failed to subscribe to events")]
    FailedToSubscribeEvents,
}