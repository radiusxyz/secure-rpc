use crate::{Args, Parser};
use secure_rpc_node_primitive::constants::rpc_url::{
    DEFAULT_BLOCKCHAIN_HTTP_URL,
    DEFAULT_BLOCKCHAIN_WS_URL,
    DEFAULT_CONTRACT_ADDRESS,
    DEFAULT_DKG_RPC_URL,
};

#[derive(Debug, Parser)]
pub enum OperatorService {
    #[command(name = "blockchain")]
    Blockchain(BlockchainOperatorArgs),
    #[command(name = "basic")]
    Basic(BasicOperatorArgs),
}

#[derive(Debug, Args)]
pub struct BlockchainOperatorArgs {
    #[doc = "Url of the blockchain node which provides the trusted setup"]
    #[clap(long = "blockchain-http-rpc-url", default_value = DEFAULT_BLOCKCHAIN_HTTP_URL)]
    pub blockchain_http_rpc_url: String,

    #[doc = "Url of the blockchain node which provides the trusted setup"]
    #[clap(long = "blockchain-ws-rpc-url", default_value = DEFAULT_BLOCKCHAIN_WS_URL)]
    pub blockchain_ws_rpc_url: String,

    #[doc = "Address of the trusted setup contract"]
    #[clap(long = "contract-address", default_value = DEFAULT_CONTRACT_ADDRESS)]
    pub contract_address: String,
}

#[derive(Debug, Args)]
pub struct BasicOperatorArgs {
    #[doc = "DKG RPC URLs for basic operator service (can specify multiple URLs separated by commas or use multiple flags)"]
    #[clap(long = "dkg-rpc-url", default_value = DEFAULT_DKG_RPC_URL, value_delimiter = ',')]
    pub dkg_rpc_urls: Vec<String>,
}