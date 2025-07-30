
use alloy::{
    primitives::Address as EthAddress, providers::{ProviderBuilder, RootProvider}, sol, sol_types::SolValue, transports::http::{reqwest::Url, Client, Http}
};
use futures::Stream;
use tokio::sync::mpsc;
pub use OperatorContract::OperatorContractInstance;
use futures_util::StreamExt;

sol! {
    #[sol(rpc)]
    contract OperatorContract {

        struct OperatorInfo {
            address account;
            string clusterRpcUrl;
            string externalRpcUrl;
        }
        
        struct TrustedSetupParams {
            string n;
            string g;
            uint32 t;
            string h;
            string max_sequencer_number;
        }
        #[derive(Debug)]
        event TrustedSetupActivated(uint256 activationBlock);
        #[derive(Debug)]
        event CommitteeActivated(uint256 activationBlock);
        
        function getOperatorList() public view returns (OperatorInfo[] memory);

        function getTrustedSetup() public view returns (bytes memory);   
    }
}

async fn spawn_event_handler<T, E>(
    mut event_stream: impl Stream<Item = Result<T, E>> + Unpin,
    blockchain_event: BlockchainEvent,
    blockchain_event_tx: mpsc::Sender<Result<BlockchainEvent, BlockchainServiceError>>
) {
    while let Some(res) = event_stream.next().await {
        if let Ok(_) = res {
            if let Err(e) = blockchain_event_tx.send(Ok(blockchain_event.clone())).await {
                tracing::error!("Failed to send {:?} event: {:?}", blockchain_event, e);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockchainEvent {
    TrustedSetupActivated,
    CommitteeActivated,
}

// TODO: Refactor to use generic wrapper
#[derive(Debug, Clone)]
pub struct BlockchainService {
    pub blockchain_event_tx: mpsc::Sender<Result<BlockchainEvent, BlockchainServiceError>>,
    pub contract_instance: OperatorContractInstance<Http<Client>, RootProvider<Http<Client>>>
}

impl BlockchainService {
    pub fn new(blockchain_http_rpc_url: String, contract_address: String) -> (Self, mpsc::Receiver<Result<BlockchainEvent, BlockchainServiceError>>) {
        let http_provider = ProviderBuilder::new().on_http(Url::parse(&blockchain_http_rpc_url).unwrap());
        let (blockchain_event_tx, blockchain_event_rx) = mpsc::channel(2);
        let contract_instance = OperatorContractInstance::new(contract_address.parse::<EthAddress>().unwrap(), http_provider);
        (Self { blockchain_event_tx, contract_instance }, blockchain_event_rx)
    }

    pub async fn get_trusted_setup(&self) -> Result<OperatorContract::TrustedSetupParams, BlockchainServiceError> 
    {
        let res = self.contract_instance.getTrustedSetup().call().await.map_err(|_| BlockchainServiceError::FailedToGetTrustedSetup)?;
        let trusted_setup = OperatorContract::TrustedSetupParams::abi_decode(&res._0.to_vec(), false).map_err(|_| BlockchainServiceError::FailedToDecodeTrustedSetup)?;
        return Ok(trusted_setup)
    }

    pub async fn get_operator_rpc_urls(&self) -> Result<Vec<String>, BlockchainServiceError> {
        let res = self.contract_instance.getOperatorList().call().await.map_err(|_| BlockchainServiceError::FailedToGetCommitteeList)?;
        let committee_rpc_urls = res._0.iter().map(|c| c.externalRpcUrl.clone()).collect();
        return Ok(committee_rpc_urls)
    }

    pub async fn subscribe_events(&self) {
        let trusted_setup_event = self.contract_instance.TrustedSetupActivated_filter().subscribe().await.map_err(|_| BlockchainServiceError::FailedToSubscribeEvents).unwrap().into_stream();
        let committee_activated_event = self.contract_instance.CommitteeActivated_filter().subscribe().await.map_err(|_| BlockchainServiceError::FailedToSubscribeEvents).unwrap().into_stream();
        let blockchain_event_tx = self.blockchain_event_tx.clone();
        tokio::spawn(spawn_event_handler(trusted_setup_event, BlockchainEvent::TrustedSetupActivated, blockchain_event_tx.clone()));
        tokio::spawn(spawn_event_handler(committee_activated_event, BlockchainEvent::CommitteeActivated, blockchain_event_tx.clone()));
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BlockchainServiceError {
    #[error("Failed to get trusted setup")]
    FailedToGetTrustedSetup,
    #[error("Failed to decode trusted setup")]
    FailedToDecodeTrustedSetup,
    #[error("Failed to get committee list")]
    FailedToGetCommitteeList,
    #[error("Failed to subscribe to events")]
    FailedToSubscribeEvents,
}