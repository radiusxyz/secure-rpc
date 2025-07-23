use alloy::{
    primitives::Address as EthAddress, 
    providers::{ProviderBuilder, RootProvider}, sol, sol_types::SolValue, transports::http::{reqwest::Url, Client, Http}
};
use secure_rpc_primitives::OperatorService;
use async_trait::async_trait;

sol! {
    #[sol(rpc)]
    contract DkgContract {

        struct CommitteeInfo {
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
        
        function getCommitteeList() public view returns (CommitteeInfo[] memory);

        function getTrustedSetup() public view returns (bytes memory);   
    }
}

#[derive(Debug, Clone)]
pub struct BlockchainService {
    contract: DkgContract::DkgContractInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl BlockchainService {
    pub fn new(url: &str, contract_address: &str) -> Self {
        let url = Url::parse(url).unwrap();
        let provider = ProviderBuilder::new()
            .on_http(url);
        let contract = DkgContract::new(contract_address.parse::<EthAddress>().unwrap(), provider);
        Self { contract }
    }

    pub async fn get_trusted_setup(&self) -> Result<DkgContract::TrustedSetupParams, BlockchainServiceError> 
    {
        let res = self.contract.getTrustedSetup().call().await.map_err(|_| BlockchainServiceError::FailedToGetTrustedSetup)?;
        DkgContract::TrustedSetupParams::abi_decode(&res._0.to_vec(), false).map_err(|_| BlockchainServiceError::FailedToDecodeTrustedSetup)
    }

    pub async fn get_committee_rpc_urls(&self) -> Result<Vec<String>, BlockchainServiceError> {
        let res = self.contract.getCommitteeList().call().await.map_err(|_| BlockchainServiceError::FailedToGetCommitteeList)?;
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

    type TrustedSetup = DkgContract::TrustedSetupParams;
    type Task = ();
    type Error = BlockchainServiceError;

    async fn get_trusted_setup(&self) -> Option<DkgContract::TrustedSetupParams> {
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