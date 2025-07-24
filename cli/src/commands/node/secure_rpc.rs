use crate::Args;
use secure_rpc_node_primitive::constants::rpc_url::{
    DEFAULT_ROLLUP_RPC_URL, 
    DEFAULT_TX_ORDERER_RPC_URL_LIST,
    DEFAULT_RPC_URL
};

#[derive(Debug, Args)]
pub struct SecureRpcArgs {
    #[doc = "Rollup id"]
    #[clap(long = "rollup-id", default_value = "0")]
    pub rollup_id: String,

    #[doc = "RPC server url"]
    #[clap(long = "rpc-url", default_value = DEFAULT_RPC_URL)]
    pub rpc_url: String,

    #[doc = "Enable encrypt mode"]
    #[clap(long = "encrypt-mode", default_value_t = true)]
    pub encrypt_mode: bool,

    #[doc = "Url of the tx-orderer node which handles transactions ordering"]
    #[clap(long = "tx-orderer-rpc-url", default_value = DEFAULT_TX_ORDERER_RPC_URL_LIST)]
    pub tx_orderer_rpc_url: String,

    #[doc = "Url of the l2 rollup node"]
    #[clap(long = "rollup-rpc-url", default_value = DEFAULT_ROLLUP_RPC_URL)]
    pub rollup_rpc_url: String,
}
