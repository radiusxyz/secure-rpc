use alloy::{
    primitives::Address as EthAddress, 
    providers::{ProviderBuilder, RootProvider}, sol, sol_types::SolValue, transports::http::{reqwest::Url, Client, Http}
};
use skde::delay_encryption::SkdeParams;

sol! {
    #[sol(rpc)]
    contract DkgContract {
        struct TrustedSetupParams {
            string n;
            string g;
            uint32 t;
            string h;
            string max_sequencer_number;
        }

        function getTrustedSetup() public view returns (bytes memory);   
    }
}

impl From<DkgContract::TrustedSetupParams> for SkdeParams {
    fn from(params: DkgContract::TrustedSetupParams) -> Self {
        Self {
            n: params.n,
            g: params.g,
            t: params.t,
            h: params.h,
            max_sequencer_number: params.max_sequencer_number,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockchainService {
    pub contract: DkgContract::DkgContractInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl BlockchainService {
    pub fn new(url: &str, contract_address: &str) -> Self {
        let url = Url::parse(url).unwrap();
        let provider = ProviderBuilder::new()
            .on_http(url);
        let contract = DkgContract::new(contract_address.parse::<EthAddress>().unwrap(), provider);
        Self { contract }
    }

    pub async fn get_trusted_setup(&self) -> Result<SkdeParams, anyhow::Error> {
        let res = self.contract.getTrustedSetup().call().await.map_err(|e| anyhow::anyhow!("Failed to get trusted setup: {}", e))?;
        let trusted_setup = DkgContract::TrustedSetupParams::abi_decode(&res._0.to_vec(), false).map_err(|e| anyhow::anyhow!("Failed to decode trusted setup: {}", e))?;
        Ok(trusted_setup.into())
    }
}
