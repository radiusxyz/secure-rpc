
use alloy::{
    primitives::Address as EthAddress, providers::{ProviderBuilder, RootProvider}, sol, transports::http::{reqwest::Url, Client, Http}
};
use secure_rpc_primitives::Operator;
use async_trait::async_trait;
pub use DkgBApp::DkgBAppInstance;

sol! {
    #[sol(rpc)]
    DkgBApp,
    "src/service/operator/ssv/contract/DkgBApp.json"
}

// TODO: Refactor to use generic wrapper
#[derive(Debug, Clone)]
pub struct BAppService(DkgBAppInstance<Http<Client>, RootProvider<Http<Client>>>);

impl BAppService {
    pub fn new(blockchain_http_rpc_url: String, contract_address: String) -> Self {
        let http_provider = ProviderBuilder::new().on_http(Url::parse(&blockchain_http_rpc_url).unwrap());
        let contract_instance = DkgBAppInstance::new(contract_address.parse::<EthAddress>().unwrap(), http_provider);
        Self(contract_instance)
    }

    pub async fn get_active_trusted_setup(&self) -> Result<DkgBApp::TrustedSetupParams, BlockchainServiceError> 
    {
        let res = self.0.getActiveTrustedSetup().call().await.map_err(|_| BlockchainServiceError::FailedToGetTrustedSetup)?;
        return Ok(res._0)
    }

    pub async fn get_operator_rpc_urls(&self) -> Result<Vec<String>, BlockchainServiceError> {
        let res = self.0.getActiveCommitteeList().call().await.map_err(|_| BlockchainServiceError::FailedToGetOperatorList)?;
        let operator_rpc_urls = res._0.iter().map(|c| c.externalRpcUrl.clone()).collect();
        return Ok(operator_rpc_urls)
    }
}

#[async_trait]
impl Operator for BAppService {
    type TrustedSetup = DkgBApp::TrustedSetupParams;
    type Error = BlockchainServiceError;

    async fn get_active_trusted_setup(&self) -> Option<Self::TrustedSetup> {
        self.get_active_trusted_setup().await.ok()
    }

    async fn get_operator_rpc_urls(&self) -> Option<Vec<String>> {
        self.get_operator_rpc_urls().await.ok()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BlockchainServiceError {
    #[error("Failed to get trusted setup")]
    FailedToGetTrustedSetup,
    #[error("Failed to decode trusted setup")]
    FailedToDecodeTrustedSetup,
    #[error("Failed to get operator list")]
    FailedToGetOperatorList,
    #[error("Failed to subscribe to events")]
    FailedToSubscribeEvents,
}