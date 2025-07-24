use alloy::{
    providers::RootProvider, sol, sol_types::SolValue, pubsub::PubSubFrontend,
    transports::http::{Client, Http},
};
use secure_rpc_primitives::OperatorService;
use async_trait::async_trait;

pub use OperatorContract::OperatorContractInstance;

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
        event TrustedSetupUpdated(uint256 currentBlock, uint256 effectiveBlock, uint256 delayRounds);
        #[derive(Debug)]
        event TrustedSetupActivated(uint256 activationBlock);
        #[derive(Debug)]
        event CommitteeActivationPending(uint256 currentBlock, uint256 effectiveBlock, uint256 delayRounds);
        #[derive(Debug)]
        event CommitteeActivated(uint256 activationBlock);
        
        function getOperatorList() public view returns (OperatorInfo[] memory);

        function getTrustedSetup() public view returns (bytes memory);   
    }
}

// TODO: Refactor to use generic wrapper
#[derive(Debug, Clone)]
pub struct BlockchainService {
    ws_provider: RootProvider<PubSubFrontend>,
    contract_instance: OperatorContractInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl BlockchainService {
    pub fn new(ws_provider: RootProvider<PubSubFrontend>, contract_instance: OperatorContractInstance<Http<Client>, RootProvider<Http<Client>>>) -> Self {
        Self { ws_provider, contract_instance }
    }

    pub async fn get_trusted_setup(&self) -> Result<OperatorContract::TrustedSetupParams, BlockchainServiceError> 
    {
        let res = self.contract_instance.getTrustedSetup().call().await.map_err(|_| BlockchainServiceError::FailedToGetTrustedSetup)?;
        OperatorContract::TrustedSetupParams::abi_decode(&res._0.to_vec(), false).map_err(|_| BlockchainServiceError::FailedToDecodeTrustedSetup)
    }

    pub async fn get_committee_rpc_urls(&self) -> Result<Vec<String>, BlockchainServiceError> {
        let res = self.contract_instance.getOperatorList().call().await.map_err(|_| BlockchainServiceError::FailedToGetCommitteeList)?;
        Ok(res._0.iter().map(|c| c.externalRpcUrl.clone()).collect())
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
}

#[async_trait]
impl OperatorService for BlockchainService {

    type TrustedSetup = OperatorContract::TrustedSetupParams;
    type Task = ();
    type Error = BlockchainServiceError;

    async fn get_trusted_setup(&self) -> Option<OperatorContract::TrustedSetupParams> {
        self.get_trusted_setup().await.ok()
    }

    async fn get_operator_rpc_urls(&self) -> Option<Vec<String>> {
        self.get_committee_rpc_urls().await.ok()
    }

    async fn create_task(&self) -> Result<Self::Task, Self::Error> {
        todo!()
    }

    async fn respond_task(&self, _task: ()) -> Result<(), Self::Error> {
        todo!()
    }
}