
use alloy::{
    primitives::Address as EthAddress, providers::{ProviderBuilder, RootProvider}, sol, sol_types::SolValue, transports::http::{Client, Http, reqwest::Url}
};
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
        event TrustedSetupActivated(uint256 activationBlock);
        #[derive(Debug)]
        event CommitteeActivated(uint256 activationBlock);
        
        function getOperatorList() public view returns (OperatorInfo[] memory);

        function getTrustedSetup() public view returns (bytes memory);   
    }
}

// TODO: Refactor to use generic wrapper
#[derive(Debug, Clone)]
pub struct BlockchainService {
    pub contract_instance: OperatorContractInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl BlockchainService {
    pub fn new(blockchain_http_rpc_url: String, contract_address: String) -> Self {
        let http_provider = ProviderBuilder::new().on_http(Url::parse(&blockchain_http_rpc_url).unwrap());
        let contract_instance = OperatorContractInstance::new(contract_address.parse::<EthAddress>().unwrap(), http_provider);
        Self { contract_instance }
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