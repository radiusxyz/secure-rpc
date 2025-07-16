use crate::Args;

#[derive(Debug, Args)]
pub struct SecureRpcArgs {
    #[doc = "Set rollup id"]
    #[clap(long = "rollup-id")]
    pub rollup_id: String,

    #[doc = "Set encrypt mode"]
    #[clap(long = "encrypt-mode")]
    pub encrypt_mode: bool,

    #[doc = "Tx-orderer RPC url"]
    #[clap(long = "tx-orderer-rpc-url")]
    pub tx_orderer_rpc_url: String,

    #[doc = "Rollup RPC url"]
    #[clap(long = "rollup-rpc-url")]
    pub rollup_rpc_url: String,

    #[doc = "DKG RPC url"]
    #[clap(long = "dkg-rpc-url")]
    pub dkg_rpc_url: String,

    #[doc = "Blockchain URL"]
    #[clap(long = "blockchain-url")]
    pub blockchain_url: String,

    #[doc = "Trusted setup contract address"]
    #[clap(long = "contract-address")]
    pub contract_address: String,
}

