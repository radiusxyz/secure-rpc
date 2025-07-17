use crate::Args;
use secure_rpc_node_primitive::constants::rpc_url::{
    DEFAULT_ROLLUP_RPC_URL, 
    DEFAULT_TX_ORDERER_RPC_URL_LIST, 
    DEFAULT_DKG_RPC_URL,
    DEFAULT_BLOCKCHAIN_URL,
    DEFAULT_CONTRACT_ADDRESS,
    DEFAULT_RPC_URL
};

#[derive(Debug, Args)]
pub struct SecureRpcArgs {
    #[doc = "Rollup id"]
    #[clap(long = "rollup-id", default_value = "0")]
    pub rollup_id: String,

    #[doc = "RPC url"]
    #[clap(long = "rpc-url", default_value = DEFAULT_RPC_URL)]
    pub rpc_url: String,

    #[doc = "Enable encrypt mode"]
    #[clap(long = "encrypt-mode")]
    pub encrypt_mode: bool,

    #[doc = "Url of the tx-orderer node which handles transactions ordering"]
    #[clap(long = "tx-orderer-rpc-url", default_value = DEFAULT_TX_ORDERER_RPC_URL_LIST)]
    pub tx_orderer_rpc_url: String,

    #[doc = "Url of the l2 rollup node"]
    #[clap(long = "rollup-rpc-url", default_value = DEFAULT_ROLLUP_RPC_URL)]
    pub rollup_rpc_url: String,

    #[doc = "Url of the DKG node which provides the encryption key"]
    #[clap(long = "dkg-rpc-url", default_value = DEFAULT_DKG_RPC_URL)]
    pub dkg_rpc_url: String,

    #[doc = "Url of the blockchain node which provides the trusted setup"]
    #[clap(long = "blockchain-url", default_value = DEFAULT_BLOCKCHAIN_URL)]
    pub blockchain_url: String,

    #[doc = "Address of the trusted setup contract"]
    #[clap(long = "contract-address", default_value = DEFAULT_CONTRACT_ADDRESS)]
    pub contract_address: String,

    #[doc = "Url of the seeder node which handles the list of urls of tx-orderer nodes"]
    #[clap(long = "seeder-url", default_value = "")]
    pub seeder_url: String,
}
